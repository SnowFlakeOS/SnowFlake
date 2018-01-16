// https://github.com/wichtounet/thor-os/tree/develop/init

#include <stdio.h>
#include <stage2/console.hpp>
#include <stage2/longmode.hpp>
#include <stage2/serial.hpp>
#include <stage2/disks.hpp>


extern "C"

[[noreturn]] void s2main(void) {
    console c;
    longmode lm;
    serial s;
    disks d;

    c.init();
    lm.init();
    s.init();
    d.init();
}
