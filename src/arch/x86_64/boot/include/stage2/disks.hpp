#ifndef DISKS_HPP
#define DISKS_HPP

#include <stddef.h>
#include <stage2/io.hpp>
#include <stage2/console.hpp>
#include <stage2/ealy_memory.hpp>
#include <stage2/longmode.hpp>

class disks {
    public:
        void init();

    private:
        console c;
        io sys;
        longmode lm;
};

#endif