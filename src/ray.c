#include "ray.h"

point3 ray_at(ray r, double t) {
    return vec_sum(r.orig, vec_mul(r.dir, t));
}