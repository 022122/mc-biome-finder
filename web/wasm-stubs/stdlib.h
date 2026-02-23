#ifndef _WASM_STDLIB_H
#define _WASM_STDLIB_H
#include <stddef.h>
void *malloc(size_t size);
void *calloc(size_t nmemb, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
int abs(int x);
void exit(int status);
#define NULL ((void*)0)
#endif