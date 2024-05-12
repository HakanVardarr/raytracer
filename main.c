#include <color.h>
#include <ray.h>
#include <vector.h>

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// clang-format off
#define ASPECT_RATIO (double)(16.0 / 9.0)
#define FOCAL_LENGTH 1.0
#define IMAGE_WIDTH 400
#define VIEWPORT_HEIGTH 2.0f

#define IMAGE_HEIGTH (int)((double)IMAGE_WIDTH / ASPECT_RATIO)
#define VIEWPORT_WIDTH (VIEWPORT_HEIGTH) * ((double)IMAGE_WIDTH / (double)IMAGE_HEIGTH)
#define CAMERA_CENTER (point3){0.0f, 0.0f, 0.0f}
#define VIEWPORT_U (vec3){VIEWPORT_WIDTH, 0.0f, 0.0f}
#define VIEWPORT_V (vec3){0.0f, -VIEWPORT_HEIGTH, 0.0f}
#define PIXEL_DELTA_U vec_div(VIEWPORT_U, IMAGE_WIDTH)
#define PIXEL_DELTA_V vec_div(VIEWPORT_V, IMAGE_HEIGTH)
#define VIEWPORT_UPPER_LEFT vec_sub(vec_sub(vec_sub(CAMERA_CENTER, (vec3){0, 0, FOCAL_LENGTH}), vec_div(VIEWPORT_U, 2.0f)), vec_div(VIEWPORT_V, 2.0f))
#define PIXEL_00 vec_sum(vec_mul(vec_sum(PIXEL_DELTA_U, PIXEL_DELTA_V), 0.5f), VIEWPORT_UPPER_LEFT)
// clang-format on

#define DOUBLE_DIV(a, b) (double)(a) / (double)(b)

void setup_ppm(FILE* outfile);
bool hit_sphere(point3 center, double radius, ray r);
color ray_color(ray r);

int main(int argc, char** argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: raytracer <out>");
        return 1;
    }

    FILE* outfile = fopen(argv[1], "w");
    if (outfile == NULL) {
        fprintf(stderr, "Cannot open file for writing: %s\n", argv[1]);
        return 1;
    }
    setup_ppm(outfile);

    for (int i = 0; i < IMAGE_HEIGTH; i++) {
        for (int j = 0; j < IMAGE_WIDTH; j++) {
            vec3 pixel_center =
                vec_sum(vec_sum(PIXEL_00, vec_mul(PIXEL_DELTA_U, j)), vec_mul(PIXEL_DELTA_V, i));
            vec3 ray_direction = vec_sub(pixel_center, CAMERA_CENTER);
            ray r = {CAMERA_CENTER, ray_direction};

            color pixel_color = ray_color(r);
            color_to_file(pixel_color, outfile);
        }
    }

    printf("Done.\n");
    return 0;
}

void setup_ppm(FILE* outfile) {
    char* buffer = (char*)malloc(32);
    sprintf(buffer, "P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGTH);
    fwrite(buffer, sizeof(buffer[0]), strlen(buffer), outfile);
    free(buffer);
}

bool hit_sphere(point3 center, double radius, ray r) {
    vec3 oc = vec_sub(center, r.orig);
    double a = vec_dot(r.dir, r.dir);
    double b = -2.0f * vec_dot(r.dir, oc);
    double c = vec_dot(oc, oc) - radius * radius;

    double discriminant = b * b - 4.0f * a * c;
    return (discriminant >= 0.0f);
}

color ray_color(ray r) {
    if (hit_sphere((point3){0.0f, 0.0f, -1.0f}, 0.5f, r)) {
        return (color){1.0f, 0.0f, 0.0f};
    }

    vec3 unit_direction = vec_unit(r.dir);
    double a = 0.5f * (unit_direction.y + 1.0f);
    return vec_sum(vec_mul((color){1.0f, 1.0f, 1.0f}, (1.0f - a)),
                   vec_mul((color){0.5f, 0.7f, 1.0f}, a));
}