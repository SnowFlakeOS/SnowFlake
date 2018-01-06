#ifndef _STDIO_H
#define _STDIO_H 1
 
#include <sys/cdefs.h>
#include <stddef.h>  

#define EOF (-1)
 
#ifdef __cplusplus
extern "C" {
#endif
 
void printf(const char * s, ...);
void vsprintf(char * str, void (*putchar)(char), const char * format, va_list arg);
void vsprintf_helper(char * str, void (*putchar)(char), const char * format, uint32_t * pos, va_list arg);
int putchar(int);
 
#ifdef __cplusplus
}
#endif
 
#endif