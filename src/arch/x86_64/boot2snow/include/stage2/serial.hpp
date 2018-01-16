//=======================================================================
// Copyright Baptiste Wicht 2013-2016.
// Distributed under the terms of the MIT License.
// (See accompanying file LICENSE or copy at
//  http://www.opensource.org/licenses/MIT)
//=======================================================================

// https://github.com/wichtounet/thor-os/tree/develop/init

#ifndef SERIAL_HPP
#define SERIAL_HPP

#include <stage2/io.hpp>
#include <stage2/console.hpp>

class serial {
    static const uint16_t COM1_PORT = 0x3f8;

    public:
        void init() {
                io sys;
                console c;

                sys.outb(COM1_PORT + 1, 0x00);    // Disable all interrupts
                sys.outb(COM1_PORT + 3, 0x80);    // Enable DLAB
                sys.outb(COM1_PORT + 0, 0x03);    // 38400 baud
                sys.outb(COM1_PORT + 1, 0x00);
                sys.outb(COM1_PORT + 3, 0x03);    // 8 bits, no parity, one stop bit
                sys.outb(COM1_PORT + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
                sys.outb(COM1_PORT + 4, 0x0B);    // IRQs enabled, RTS/DSR set

                c.log("Serial initialized!");
        }
};

#endif