#ifndef __VECTOR_H
#define __VECTOR_H

typedef struct {
    double x, y, z;
} vec3;

typedef vec3 point3;

void vec_sum_m(vec3* v1, vec3 v2);
void vec_mul_m(vec3* v, double t);
void vec_div_m(vec3* v, double t);

vec3 vec_negate(vec3 v);
vec3 vec_sum(vec3 v1, vec3 v2);
vec3 vec_mul(vec3 v1, double t);

double length(vec3 v);
double length_squared(vec3 v);
double vec_dot(vec3 v1, vec3 v2);

char* vec_string(vec3 v);

#endif