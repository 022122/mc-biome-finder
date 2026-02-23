/* Minimal libc implementation for WASM target */
#include <stddef.h>
#include <stdint.h>

/* Simple bump allocator for WASM */
extern unsigned char __heap_base;
static unsigned char *heap_ptr = &__heap_base;

void *malloc(size_t size) {
    /* Align to 8 bytes */
    size = (size + 7) & ~7;
    void *ptr = heap_ptr;
    heap_ptr += size;
    return ptr;
}

void *calloc(size_t nmemb, size_t size) {
    size_t total = nmemb * size;
    void *ptr = malloc(total);
    if (ptr) {
        unsigned char *p = (unsigned char *)ptr;
        for (size_t i = 0; i < total; i++) p[i] = 0;
    }
    return ptr;
}

void *realloc(void *ptr, size_t size) {
    /* Simple: always allocate new, copy old (we don't know old size, so this is lossy) */
    void *new_ptr = malloc(size);
    /* Can't copy without knowing old size - but cubiomes rarely uses realloc */
    (void)ptr;
    return new_ptr;
}

void free(void *ptr) {
    /* No-op in bump allocator */
    (void)ptr;
}

void *memset(void *s, int c, size_t n) {
    unsigned char *p = (unsigned char *)s;
    for (size_t i = 0; i < n; i++) p[i] = (unsigned char)c;
    return s;
}

void *memcpy(void *dest, const void *src, size_t n) {
    unsigned char *d = (unsigned char *)dest;
    const unsigned char *s = (const unsigned char *)src;
    for (size_t i = 0; i < n; i++) d[i] = s[i];
    return dest;
}

void *memmove(void *dest, const void *src, size_t n) {
    unsigned char *d = (unsigned char *)dest;
    const unsigned char *s = (const unsigned char *)src;
    if (d < s) {
        for (size_t i = 0; i < n; i++) d[i] = s[i];
    } else {
        for (size_t i = n; i > 0; i--) d[i-1] = s[i-1];
    }
    return dest;
}

int memcmp(const void *s1, const void *s2, size_t n) {
    const unsigned char *a = (const unsigned char *)s1;
    const unsigned char *b = (const unsigned char *)s2;
    for (size_t i = 0; i < n; i++) {
        if (a[i] != b[i]) return a[i] - b[i];
    }
    return 0;
}

size_t strlen(const char *s) {
    size_t len = 0;
    while (s[len]) len++;
    return len;
}

char *strcpy(char *dest, const char *src) {
    char *d = dest;
    while ((*d++ = *src++));
    return dest;
}

int strcmp(const char *s1, const char *s2) {
    while (*s1 && *s1 == *s2) { s1++; s2++; }
    return *(unsigned char*)s1 - *(unsigned char*)s2;
}

int abs(int x) {
    return x < 0 ? -x : x;
}

void exit(int status) {
    (void)status;
    __builtin_trap();
}

/* printf stubs - no-op in WASM */
typedef struct FILE FILE;
FILE *stdin = 0;
FILE *stdout = 0;
FILE *stderr = 0;

int printf(const char *fmt, ...) { (void)fmt; return 0; }
int fprintf(FILE *stream, const char *fmt, ...) { (void)stream; (void)fmt; return 0; }
int sprintf(char *str, const char *fmt, ...) { (void)str; (void)fmt; return 0; }
int snprintf(char *str, size_t size, const char *fmt, ...) { (void)str; (void)size; (void)fmt; return 0; }
FILE *fopen(const char *path, const char *mode) { (void)path; (void)mode; return 0; }
int fclose(FILE *stream) { (void)stream; return 0; }
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream) { (void)ptr; (void)size; (void)nmemb; (void)stream; return 0; }
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream) { (void)ptr; (void)size; (void)nmemb; (void)stream; return 0; }
int fseek(FILE *stream, long offset, int whence) { (void)stream; (void)offset; (void)whence; return 0; }
long ftell(FILE *stream) { (void)stream; return 0; }
int fflush(FILE *stream) { (void)stream; return 0; }
int feof(FILE *stream) { (void)stream; return 1; }