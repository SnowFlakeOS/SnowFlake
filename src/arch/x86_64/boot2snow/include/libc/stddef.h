#ifndef STDDEF_H
#define STDDEF_H

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;

typedef uint64_t size_t;

typedef char *va_list;
#define intsizeof(n)    ((sizeof(n) + sizeof(int) - 1) &~(sizeof(int) - 1))
#define va_start(ap, v) (ap = (va_list)&(v) + intsizeof(v))
#define va_arg(ap, t)   (*(t *) ((ap += intsizeof(t)) - intsizeof(t)))
#define va_end(ap)      (ap = (va_list)0)

#define NULL ((void*)0)

#endif