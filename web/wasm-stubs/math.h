#ifndef _WASM_MATH_H
#define _WASM_MATH_H
double floor(double x);
double ceil(double x);
double sqrt(double x);
double pow(double base, double exp);
double fabs(double x);
double sin(double x);
double cos(double x);
double round(double x);
double log(double x);
double exp(double x);
double fmod(double x, double y);
float floorf(float x);
float sqrtf(float x);
float fabsf(float x);
float sinf(float x);
float cosf(float x);
float roundf(float x);
double nan(const char *tagp);
float nanf(const char *tagp);
#define INFINITY __builtin_inf()
#define NAN __builtin_nan("")
#define HUGE_VAL __builtin_huge_val()
#endif