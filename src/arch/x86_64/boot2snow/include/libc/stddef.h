#ifndef STDDEF_H
#define STDDEF_H

typedef unsigned int uint8_t __attribute__((__mode__(__QI__))); ///< An unsigned 8-bit number
typedef unsigned int uint16_t __attribute__ ((__mode__ (__HI__))); ///< An unsigned 16-bit number
typedef unsigned int uint32_t __attribute__ ((__mode__ (__SI__))); ///< An unsigned 32-bit number
typedef unsigned int uint64_t __attribute__ ((__mode__ (__DI__))); ///< An unsigned 64-bit number

typedef int int8_t __attribute__((__mode__(__QI__))); ///< A signed 8-bit number
typedef int int16_t __attribute__ ((__mode__ (__HI__))); ///< A signed 16-bit number
typedef int int32_t __attribute__ ((__mode__ (__SI__))); ///< A signed 32-bit number
typedef int int64_t __attribute__ ((__mode__ (__DI__))); ///< A signed 64-bit number

//typedef uint64_t uintptr_t; ///< Type that can be used to store a pointer value
typedef uint64_t size_t; ///< Type that can be used to store the size of a collectiotn

typedef long int ptrdiff_t;

typedef char *va_list;
#define intsizeof(n)    ((sizeof(n) + sizeof(int) - 1) &~(sizeof(int) - 1))
#define va_start(ap, v) (ap = (va_list)&(v) + intsizeof(v))
#define va_arg(ap, t)   (*(t *) ((ap += intsizeof(t)) - intsizeof(t)))
#define va_end(ap)      (ap = (va_list)0)

#define NULL ((void*)0)

#endif