#ifndef __COLOR_H
#define __COLOR_H

#include <stdio.h>
#include <vector.h>

typedef vec3 color;

void color_to_file(color c, FILE* f);

#endif