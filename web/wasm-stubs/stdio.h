#ifndef _WASM_STDIO_H
#define _WASM_STDIO_H
#include <stddef.h>
typedef struct FILE FILE;
extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;
int printf(const char *fmt, ...);
int fprintf(FILE *stream, const char *fmt, ...);
int sprintf(char *str, const char *fmt, ...);
int snprintf(char *str, size_t size, const char *fmt, ...);
FILE *fopen(const char *path, const char *mode);
int fclose(FILE *stream);
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream);
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream);
int fseek(FILE *stream, long offset, int whence);
long ftell(FILE *stream);
int fflush(FILE *stream);
int feof(FILE *stream);
#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2
#define EOF (-1)
#endif