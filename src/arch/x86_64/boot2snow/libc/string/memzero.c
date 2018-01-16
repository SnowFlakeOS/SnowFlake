#include <string.h>

void memzero(void* ptr, size_t size)
{
    if (size % 8 == 0) {
        size /= 8;

        uint64_t* p64 = ptr;

        for (uint64_t i = 0; i < size; i++) {
            p64[i] = 0;
        }
    } else {
        uint8_t* p8 = ptr;

        for (size_t i = 0; i < size; i++) {
            p8[i] = 0;
        }
    }
}