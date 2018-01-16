#include <string.h>

int strcpy(char *dst,const char *src)
{
    int i = 0;
    while ((*dst++ = *src++) != 0)
        i++;
    return i;
}
