# Skeleton:
# https://github.com/crystal-lang/crystal/blob/master/src/gc/none.cr

module GC
  def self.init
  end

  def self.malloc(size : UInt32)
    Heap.kalloc size
  end

  def self.malloc_atomic(size : UInt32)
    Heap.kalloc size
  end

  def self.realloc(ptr : Void*, size : UInt32)
    Heap.realloc ptr, size
  end

  def self.collect
  end

  def self.enable
  end

  def self.disable
  end

  def self.free(ptr : Void*)
    Heap.free ptr
  end

  def self.is_heap_ptr(ptr : Void*)
    false
  end

  def self.add_finalizer(object)
  end
end
