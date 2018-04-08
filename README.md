# ![SnowFlake](./logo.png)

[![BSD-3-Clause][s1]][li]
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FSnowFlakeOS%2FSnowFlake.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2FSnowFlakeOS%2FSnowFlake?ref=badge_shield)

[s1]: https://img.shields.io/badge/License-BSD%203--Clause-blue.svg

[li]: LICENSE

Technology is free, SnowFlakeOS

## Library used
- utf16_literal (https://github.com/thepowersgang/rust_os/tree/master/Bootloaders/libuefi/utf16_literal)
- uefi (forked, https://github.com/thepowersgang/rust_os/tree/master/Bootloaders/libuefi)

## TODO
### Boot2Snow (x86_64, UEFI)
- [x] Basical UI (from https://github.com/system76/firmware-update)
- [x] Load kernel from disk
- [ ] Set virtual memory map
### SnowKernel
- [ ] Better GUI library support
- [ ] Write elf loader
- [ ] Write I/O drivers (keyboard, mouse, etc.)
- [ ] Multitasking support
- [ ] Write syscall
- [ ] Write init process
### FlakeOS

## Building
Requirements to build
- Rust (https://www.rust-lang.org)
- NASM (http://www.nasm.us/)
- GCC Toolchain or GCC (https://gcc.gnu.org/)

### Windows
Will be added later

### Mac
Will be added later

### Linux
To build SnowFlake as an x86_64 target, x86_64-elf cross-compilation is required.
If you do not have the x86_64-elf compiler, and your system is x86_64, you can use the 'x86_64-linux_env.sh' script.
```
$ sh x86_64-linux_env.sh
```
#### Arch Linux
```
$ pacman -S qemu nasm mtools
$ git clone https://github.com/SnowFlake/SnowFlake.git
$ cd SnowWhite
$ make run
```

## Reference
- https://github.com/phil-opp/blog_os
- https://github.com/thepowersgang/rust_os
- https://github.com/redox-os/orbclient
- https://github.com/redox-os/uefi
- https://github.com/system76/firmware-update


## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FSnowFlakeOS%2FSnowFlake.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2FSnowFlakeOS%2FSnowFlake?ref=badge_large)