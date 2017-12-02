# SnowFlake
SnowFlake, this is an Operating System and is written in Rust.

# Building
This is SnowFlake is require for build.
- Rust (https://www.rust-lang.org)
- NASM (http://www.nasm.us/)
- GCC Toolchain or GCC (https://gcc.gnu.org/)
- Clang (https://clang.llvm.org/)

## On Windows
I will add later

## On Mac
macOS is default ld is bsd ld (can not link SnowFlake)\
and default as is clang too (build error)\
If you want build SnowFlake on macOS\
- Need HomeBrew (https://brew.sh/)
- Need Xcode Command Line Tools (This will install both HomeBrew)
- Need NASM (can install in HomeBrew)
- Need QEMU (if you want run SnowFlake)
```
$ /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
$ homebrew install nasm qemu
$ git clone https://github.com/SnowFlake/mac-binutils-script.git
$ cd mac-binutils-script
$ sudo ./compile.sh
$ git clone https://github.com/SnowFlake/SnowFlake.git
$ cd SnowWhiteOS
$ make run
```

## On Linux
### Fedora
I will add later

# Thanks for
- https://stackoverflow.com/questions/27051471/call-c-kernel-from-assembly-bootloader/33263223#33263223
- https://github.com/rzhikharevich/swift-bare-bones
- https://github.com/klange/taylor
- https://github.com/charliesome/mini64
- https://github.com/apple/swift
- https://github.com/phil-opp/blog_os
- 