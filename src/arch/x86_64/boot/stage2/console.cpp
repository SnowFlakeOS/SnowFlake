// https://github.com/wichtounet/thor-os/tree/develop/init

#include <stage2/console.hpp>

void console::log(const char* message) {
    printf("%s %s\n", LOG_HEAD, message);
}

void console::error(const char* message) {
    printf("%s %s\n", ERROR_HEAD, message);
    asm volatile("cli; hlt");
    __builtin_unreachable();
}

void console::val(const char* message, uint32_t value) {
    printf("%s %s %d\n", LOG_HEAD, message, value);
}
