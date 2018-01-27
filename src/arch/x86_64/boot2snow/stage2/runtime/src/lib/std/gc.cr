# Reference:
# https://github.com/crystal-lang/crystal/blob/master/src/gc.cr

fun __crystal_malloc(size : UInt32) : Void*
  GC.malloc size
end

fun __crystal_malloc_atomic(size : UInt32) : Void*
  GC.malloc_atomic size
end

fun __crystal_realloc(pointer : Void*, size : UInt32) : Void*
  GC.realloc pointer, size
end

fun __crystal_malloc64(size : UInt64) : Void*
  GC.malloc size
end

fun __crystal_malloc_atomic64(size : UInt64) : Void*
  GC.malloc_atomic size
end

fun __crystal_realloc64(ptr : Void*, size : UInt64) : Void*
  GC.realloc ptr, size
end

module GC
  def self.malloc(size : Int)
    malloc size
  end

  def self.malloc_atomic(size : Int)
    malloc_atomic size
  end

  def self.realloc(pointer : Void*, size : Int)
    realloc pointer, size
  end
end

require "./gc/minimal"
