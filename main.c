#include <color.h>
#include <ray.h>
#include <vector.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define IMAGE_WIDTH 256
#define IMAGE_HEIGTH 256

#define DOUBLE_DIV(a, b) (double)(a) / (double)(b)

void setup_ppm(FILE* outfile);

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
            color c = {DOUBLE_DIV(j, IMAGE_WIDTH - 1), DOUBLE_DIV(i, IMAGE_WIDTH - 1), 0.0f};
            color_to_file(c, outfile);
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