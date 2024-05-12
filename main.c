#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define IMAGE_WIDTH 256
#define IMAGE_HEIGTH 256

int main(int argc, char** argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: raytracer <out>");
        return 1;
    }

    FILE* outfile;
    char* buffer;

    outfile = fopen(argv[1], "w");
    if (outfile == NULL) {
        fprintf(stderr, "Cannot open file for writing: %s\n", argv[1]);
        return 1;
    }

    buffer = (char*)malloc(255);

    sprintf(buffer, "P3\n%d %d\n255\n", IMAGE_WIDTH, IMAGE_HEIGTH);
    fwrite(buffer, sizeof(buffer[0]), strlen(buffer), outfile);

    for (int i = 0; i < IMAGE_HEIGTH; i++) {
        for (int j = 0; j < IMAGE_WIDTH; j++) {
            int r = (int)(255.999 * (double)j / (double)(IMAGE_WIDTH - 1));
            int g = (int)(255.999 * (double)i / (double)(IMAGE_HEIGTH - 1));
            int b = (int)(255.999 * 0.0f);

            sprintf(buffer, "%d %d %d\n", r, g, b);
            fwrite(buffer, sizeof(buffer[0]), strlen(buffer), outfile);
        }
    }

    free(buffer);
    printf("Done.\n");

    return 0;
}