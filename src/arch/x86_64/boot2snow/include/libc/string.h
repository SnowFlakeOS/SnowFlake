#ifndef _STRING_H
#define _STRING_H 1
 
#include <sys/cdefs.h>
 
#include <stddef.h>
 
#ifdef __cplusplus
extern "C" {
#endif
 
int memcmp(const void*, const void*, size_t);
int atoi(char * string);
int strcpy(char *dst,const char *src);
int isspace(char c);
void memzero(void* ptr, size_t size);
void* memcpy(void* __restrict, const void* __restrict, size_t);
void* memmove(void*, const void*, size_t);
void* memset(void*, int, size_t);
void itoa(char *buf, unsigned long int n, int base);
size_t strlen(const char*);
char *strncpy(char *dest, const char *src, size_t n);
 
#ifdef __cplusplus
}
#endif
 
#endif