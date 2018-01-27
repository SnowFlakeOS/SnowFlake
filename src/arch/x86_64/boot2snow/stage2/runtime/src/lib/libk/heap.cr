# Alignment constants
private ADDRESS_ALIGNMENT   = 4_u32 # Align to 32 bits
private BLOCK_SIZE_MULTIPLE = 4_u32 # Align to 32 bits

# Guards to check for heap corruption
private GUARD1 = 0x464c554f_u32 # FLUO
private GUARD2 = 0x52495445_u32 # RITE

# Alias block to make it easer to use
private alias Block = LibHeap::Block

# Low level stuff
private lib LibHeap
  @[Packed]
  struct Block
    bsize : UInt32
    dsize : UInt32
    bnext : Block*
    bdata : UInt8*
  end
end

# Generic heap allocator
struct HeapAllocator(T)

  # Allocates a block of uninitialized memory.
  def self.kalloc : T*

    # Allocate memory
    block = Heap.kalloc sizeof(T).to_u32

    # Cast allocated block to target type
    block.as T*
  end

  # Allocates a block of zero-filled memory.
  def self.calloc : T*
    
    # Allocate zeroed memory
    block = Heap.calloc sizeof(T).to_u32

    # Cast allocated block to target type
    block.as T*
  end

  # Reallocates a pointer.
  def self.realloc(ptr : _*, size : UInt32) : T*

    # Reallocate memory
    block = Heap.realloc ptr, size

    # Cast block to target type
    block.as T*
  end

  # Reallocates a pointer.
  def self.realloc(ptr : _*) : T*

    # Reallocate memory
    self.realloc ptr, sizeof(T).to_u32
  end
end

module Heap
  extend self

  # Module variables
  @@next_addr = Pointer(UInt8).null
  @@next_free = Pointer(Block).null
  @@last_used = Pointer(Block).null

  def init(end_of_kernel : UInt32)

    # Align starting address
    next_addr_aligned = align_address end_of_kernel

    # Create pointer to aligned starting address
    @@next_addr = Pointer(UInt8).new next_addr_aligned.to_u64
  end

  # Validates the heap.
  # Iterates over all used blocks and tests guard integrity.
  def validate
    
    # Get the last used block
    current_block = @@last_used

    # Loop while the current block is valid
    while current_block

      # Validate the block
      validate_block current_block

      # Get the next block
      current_block = current_block.value.bnext
    end
  end

  # Allocates a block of uninitialized memory.
  def kalloc(size : UInt32) : Void*

    # Allocate a block
    block = alloc size

    # Return the user data
    block.value.bdata.to_void_ptr
  end

  # Allocates a block of zero-filled memory.
  def calloc(size : UInt32) : Void*

    # Allocate a block
    block = alloc size

    # Zero-fill the block
    LibK.memset block.value.bdata, 0_u8, block.value.bsize

    # Return the user data
    block.value.bdata.to_void_ptr
  end

  # Allocates a block.
  private def alloc(size : UInt32) : Block*

    # Try to find an existing block of sufficient size.
    # If that fails, allocate a new block.
    block = find_or_alloc_block size

    # Raise if the block is invalid
    raise "Unable to allocate memory!" unless block

    # Link the block to the last used one
    block.value.bnext = @@last_used

    # Mark this block as the last used one
    @@last_used = block

    # Return the block
    block
  end

  # Tries to find a block of sufficient size.
  # If no block is found, a new one is allocated.
  private def find_or_alloc_block(size : UInt32) : Block* | Nil
    
    # Try finding a block of sufficient size
    block = find_fitting_block size

    # If no block was found
    unless block

      # Allocate space for a new block structure
      block_size = align_block_size sizeof(Block).to_u32
      block_data = alloc_block block_size

      # Cast the allocated space to a block pointer
      block = block_data.as Block*

      # Return Nil if the allocation failed
      return unless block

      # Align the block size to the block size multiple
      size_aligned = align_block_size size
      
      # Set the block size
      block.value.bsize = size_aligned
      block.value.dsize = size

      # Allocate the actual data block
      user_data = alloc_block size_aligned

      # If the data block allocation failed
      unless user_data

        # Deallocate the previously allocated block
        block_size_total_aligned = calc_block_size block_size
        @@next_addr -= block_size_total_aligned.to_i32

        # Return Nil
        return
      end

      # Set the block data pointer
      block.value.bdata = user_data
    end

    # Return the block
    block
  end

  # Attempts to find a fitting block.
  # Uses best-fit linear search.
  private def find_fitting_block(size : UInt32) : Block* | Nil

    # Get the next free block
    current_block = @@next_free
    last_block = current_block

    # Loop while the current block is valid
    while current_block

      # Get the block
      block = current_block.value

      # Test if the block fits
      if block.dsize == size

        # Link the other blocks back together
        if current_block == last_block
          @@next_free = block.bnext
        else
          last_block.value.bnext = block.bnext
        end

        # Return the block
        return current_block
      end

      # Mark this block as the last one
      last_block = current_block
      
      # Get the next block
      current_block = block.bnext
    end
  end

  # Allocates a raw data block.
  # Adds guards before and after the block.
  # Returns the data chunk.
  private def alloc_block(size : UInt32) : UInt8*

    # Get the next unused address
    addr = @@next_addr

    # Write the first guard
    addr.as(UInt32*).value = GUARD1

    # Get the start of the data block
    addr_data_start = addr.offset sizeof(GUARD1)

    # Skip behind the data block
    addr = addr_data_start.offset size

    # Write the second guard
    addr.as(UInt32*).value = GUARD2

    # Offset the next unused address
    @@next_addr += calc_block_size size
    
    # Return the allocated data block
    addr_data_start
  end

  def realloc(ptr : _*, size : UInt32) : Void*

    # Test if the ptr is a null pointer
    if ptr.null?

      # Allocate a new block
      block = kalloc size

      # Return the newly allocated block
      return block.to_void_ptr
    end

    # Try finding the associated block
    block = find_associated_block ptr

    # Return a null pointer if the pointer is unmanaged
    return Pointer(Void).null unless block

    # Test if the requested size is 0
    if size == 0

      # Free the block
      free ptr

      # Return a null pointer
      return Pointer(Void).null
    end

    # Get the block size
    block_size = block.value.bsize

    # Test if the block still fits
    if block_size <= size

      # Return the current block
      ptr.to_void_ptr
    else

      # Free the old block
      free ptr

      # Allocate a new zero-filled block
      new_block = calloc size

      # Copy the data to the new block
      LibK.memmove new_block, ptr, block.value.dsize

      # Return the new block
      new_block.to_void_ptr
    end
  end

  def free(ptr : _*)

    # Get the last used block
    current_block = @@last_used
    last_block = current_block

    # Loop while the current block is valid
    while current_block

      # Get the block
      block = current_block.value

      # Test if the data block equals the block to be freed
      if block.bdata == ptr

        # Validate the user data
        validate_block current_block

        # Link the other blocks back together
        if current_block == last_block
          @@last_used = block.bnext
        else
          last_block.value.bnext = block.bnext
        end

        # Mark the block as free
        @@next_free = current_block
      end

      # Mark this block as the last one
      last_block = current_block

      # Get the next block
      current_block = block.bnext
    end
  end

  # Finds the associated block header of a pointer.
  private def find_associated_block(ptr : _*) : Block* | Nil

    # Cast the ptr to Void*
    ptr = ptr.to_void_ptr
    
    # Get the last used block
    current_block = @@last_used

    # Loop while the current block is valid
    while current_block

      # Get the block
      block = current_block.value

      # Test if the data block equals the target block
      if block.bdata == ptr

        # Return the associated block
        return current_block
      end

      # Get the next block
      current_block = block.bnext
    end
  end

  # Validates the integrity of a data block.
  private def validate_block(block : Block*)

    # Get the block
    block = block.value

    # Get the guard values
    guard1 = (block.bdata - sizeof(GUARD1)).as(UInt32*).value
    guard2 = (block.bdata + block.bsize).as(UInt32*).value

    # Validate guard integrity
    raise "HEAP_VALIDATE_FAIL_GUARD1" unless guard1 == GUARD1
    raise "HEAP_VALIDATE_FAIL_GUARD2" unless guard2 == GUARD2
  end

  # Calculates the total size of a block.
  # Includes guards and data chunk.
  @[AlwaysInline]
  private def calc_block_size(size : UInt32) : UInt32
    size + sizeof(GUARD1) + sizeof(GUARD2)
  end

  # Rounds a number up to a specific nearest multiple.
  @[AlwaysInline]
  private def align(num : UInt32, multiple : UInt32) : UInt32
    t = num + multiple - 1
    t - t % multiple
  end

  # Aligns an address.
  @[AlwaysInline]
  private def align_address(addr : UInt32) : UInt32
    align addr, ADDRESS_ALIGNMENT
  end

  # Aligns the size of a block.
  @[AlwaysInline]
  private def align_block_size(size : UInt32) : UInt32
    align size, BLOCK_SIZE_MULTIPLE
  end
end
