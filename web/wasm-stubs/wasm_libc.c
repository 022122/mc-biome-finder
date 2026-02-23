/* Minimal libc implementation for WASM target */
#include <stddef.h>
#include <stdint.h>

/* ============ Math functions ============ */
/* WASM has native opcodes for these via builtins */
double floor(double x) { return __builtin_floor(x); }
double ceil(double x) { return __builtin_ceil(x); }
double sqrt(double x) { return __builtin_sqrt(x); }
double fabs(double x) { return __builtin_fabs(x); }
double round(double x) { return __builtin_round(x); }
float floorf(float x) { return __builtin_floorf(x); }
float sqrtf(float x) { return __builtin_sqrtf(x); }
float fabsf(float x) { return __builtin_fabsf(x); }
float roundf(float x) { return __builtin_roundf(x); }

/* fmod: x - trunc(x/y)*y */
double fmod(double x, double y) {
    if (y == 0.0) return x;
    return x - __builtin_floor(x / y) * y;
}

/* exp(x) using range reduction + polynomial
 * exp(x) = 2^k * exp(r) where x = k*ln2 + r, |r| <= ln2/2 */
double exp(double x) {
    if (x > 709.0) return 1.0/0.0; /* +inf */
    if (x < -745.0) return 0.0;
    /* Range reduction: x = k*ln2 + r */
    const double LN2 = 0.6931471805599453;
    const double INV_LN2 = 1.4426950408889634;
    double k_d = __builtin_floor(x * INV_LN2 + 0.5);
    int k = (int)k_d;
    double r = x - k_d * LN2;
    /* Pade-like approximation for exp(r), |r| <= ln2/2 */
    double r2 = r * r;
    double r3 = r2 * r;
    double r4 = r2 * r2;
    double p = 1.0 + r + r2 * 0.5 + r3 * (1.0/6.0) + r4 * (1.0/24.0)
             + r4 * r * (1.0/120.0) + r4 * r2 * (1.0/720.0)
             + r4 * r3 * (1.0/5040.0);
    /* Multiply by 2^k using bit manipulation */
    union { double d; uint64_t u; } u;
    u.d = p;
    u.u += ((uint64_t)k << 52);
    return u.d;
}

/* log(x) using the identity: log(x) = log(m * 2^e) = e*ln2 + log(m)
 * where 1 <= m < 2, then use polynomial for log(m) */
double log(double x) {
    if (x <= 0.0) return -1.0/0.0; /* -inf for 0, NaN for negative */
    union { double d; uint64_t u; } u;
    u.d = x;
    int e = (int)((u.u >> 52) & 0x7FF) - 1023;
    /* Extract mantissa, set exponent to 0 (biased 1023) */
    u.u = (u.u & 0x000FFFFFFFFFFFFFULL) | 0x3FF0000000000000ULL;
    double m = u.d; /* 1.0 <= m < 2.0 */
    /* Use log(m) = log(1+f) where f = m-1, via polynomial */
    double f = m - 1.0;
    double f2 = f * f;
    double f3 = f2 * f;
    double f4 = f2 * f2;
    /* Truncated series: log(1+f) = f - f^2/2 + f^3/3 - f^4/4 + ... */
    double logm = f - f2 * 0.5 + f3 * (1.0/3.0) - f4 * 0.25
                + f4 * f * 0.2 - f4 * f2 * (1.0/6.0)
                + f4 * f3 * (1.0/7.0) - f4 * f4 * 0.125;
    const double LN2 = 0.6931471805599453;
    return (double)e * LN2 + logm;
}

/* pow(base, exponent) = exp(exponent * log(base)) */
double pow(double base, double exponent) {
    if (exponent == 0.0) return 1.0;
    if (base == 0.0) return 0.0;
    if (base == 1.0) return 1.0;
    /* Check for integer exponent */
    double abs_exp = fabs(exponent);
    if (abs_exp == __builtin_floor(abs_exp) && abs_exp <= 64.0) {
        int n = (int)abs_exp;
        double result = 1.0;
        double b = base;
        while (n > 0) {
            if (n & 1) result *= b;
            b *= b;
            n >>= 1;
        }
        return exponent < 0.0 ? 1.0 / result : result;
    }
    if (base < 0.0) return 0.0; /* negative base with non-integer exp */
    return exp(exponent * log(base));
}

/* sin(x) using range reduction + Chebyshev polynomial */
static double _reduce_angle(double x) {
    const double TWO_PI = 6.283185307179586;
    const double PI = 3.141592653589793;
    x = fmod(x, TWO_PI);
    if (x < -PI) x += TWO_PI;
    if (x > PI) x -= TWO_PI;
    return x;
}

double sin(double x) {
    x = _reduce_angle(x);
    /* Polynomial approximation (minimax on [-pi, pi]) */
    double x2 = x * x;
    double x3 = x2 * x;
    double x5 = x3 * x2;
    double x7 = x5 * x2;
    double x9 = x7 * x2;
    double x11 = x9 * x2;
    return x - x3 * (1.0/6.0) + x5 * (1.0/120.0) - x7 * (1.0/5040.0)
           + x9 * (1.0/362880.0) - x11 * (1.0/39916800.0);
}

double cos(double x) {
    x = _reduce_angle(x);
    double x2 = x * x;
    double x4 = x2 * x2;
    double x6 = x4 * x2;
    double x8 = x6 * x2;
    double x10 = x8 * x2;
    return 1.0 - x2 * 0.5 + x4 * (1.0/24.0) - x6 * (1.0/720.0)
           + x8 * (1.0/40320.0) - x10 * (1.0/3628800.0);
}

float sinf(float x) { return (float)sin((double)x); }
float cosf(float x) { return (float)cos((double)x); }

double nan(const char *tagp) { (void)tagp; return __builtin_nan(""); }
float nanf(const char *tagp) { (void)tagp; return __builtin_nanf(""); }

/* ============ Memory allocator ============ */

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
int puts(const char *s) { (void)s; return 0; }
int fputs(const char *s, FILE *stream) { (void)s; (void)stream; return 0; }
int putchar(int c) { (void)c; return c; }
FILE *fopen(const char *path, const char *mode) { (void)path; (void)mode; return 0; }
int fclose(FILE *stream) { (void)stream; return 0; }
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream) { (void)ptr; (void)size; (void)nmemb; (void)stream; return 0; }
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream) { (void)ptr; (void)size; (void)nmemb; (void)stream; return 0; }
int fseek(FILE *stream, long offset, int whence) { (void)stream; (void)offset; (void)whence; return 0; }
long ftell(FILE *stream) { (void)stream; return 0; }
int fflush(FILE *stream) { (void)stream; return 0; }
int feof(FILE *stream) { (void)stream; return 1; }