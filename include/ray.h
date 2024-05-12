#ifndef __RAY_H
#define __RAY_H

#include <vector.h>

typedef struct {
    point3 orig;
    vec3 dir;
} ray;

point3 ray_at(ray r, double t);

#endif