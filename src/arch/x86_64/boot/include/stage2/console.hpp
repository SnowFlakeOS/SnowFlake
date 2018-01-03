#ifndef CONSOLE_HPP
#define CONSOLE_HPP

#define LOG_HEAD "[FLAKE]"
#define ERROR_HEAD "[ERROR]"

#include <stdio.h>
#include <stddef.h>
#include <console/tty.h>

class console {
    public:
        void init() {
            terminal_initialize();
            log("Terminal initialized!");
        }

        void log(const char* message);
        void error(const char* message);
        void val(const char* message, uint32_t value);
};

#endif