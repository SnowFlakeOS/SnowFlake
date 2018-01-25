
# This stuff is required by the Crystal runtime

fun memset(ptr : Void*, val : UInt8, count : UInt32)
    LibK.memset ptr, val, count
  end