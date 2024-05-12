#include <color.h>
#include <stdlib.h>
#include <string.h>

void color_to_file(color c, FILE* f) {
    vec_mul_m(&c, 255.999f);
    char* color_string = vec_string(c);
    fwrite(color_string, sizeof(color_string[0]), strlen(color_string), f);
    free(color_string);
}