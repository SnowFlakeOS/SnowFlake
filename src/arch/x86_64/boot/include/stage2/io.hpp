#ifndef IO_HPP
#define IO_HPP

#include <stddef.h>

class io {
    public:
        static inline void outb(uint8_t value, uint16_t port)
        {
            asm volatile("out %1, %0" : : "a" (value), "dN" (port));
        }

        static inline uint8_t inb(uint16_t port)
        {
            uint8_t ret;
            asm volatile ( "in %0, %1"
                                : "=a"(ret)
                                : "dN"(port) );
            return ret;
        }

        static inline uint16_t inw(uint16_t _port){
            uint16_t rv;

            asm volatile ("in %[data], %[port]"
                : [data] "=a" (rv)
                : [port] "dN" (_port));

            return rv;
        }
};

#endif