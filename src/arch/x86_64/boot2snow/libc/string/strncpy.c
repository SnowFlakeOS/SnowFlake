#include <string.h>

char *strncpy(char *dest, const char *src, size_t n)
{
   char *temp = dest;
   while (n-- && (*dest++ = *src++))
    ;
   return temp;
}
