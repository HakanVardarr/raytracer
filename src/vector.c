#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <vector.h>

void vec_sum(vec3* v1, vec3 v2) {
    v1->x += v2.x;
    v1->y += v2.y;
    v1->z += v2.z;
}

void vec_mul(vec3* v, double t) {
    v->x *= t;
    v->y *= t;
    v->z *= t;
}

void vec_div(vec3* v, double t) {
    vec_mul(v, 1.0 / t);
}

vec3 vec_negate(vec3 v) {
    return (vec3){-v.x, -v.y, -v.z};
}

double length(vec3 v) {
    return sqrt(length_squared(v));
}

double length_squared(vec3 v) {
    return v.x * v.x + v.y * v.y + v.z * v.z;
}

double vec_dot(vec3 v1, vec3 v2) {
    return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
}

char* vec_string(vec3 v) {
    char* buffer = (char*)malloc(32);
    sprintf(buffer, "%f %f %f\n", v.x, v.y, v.z);
    return buffer;
}