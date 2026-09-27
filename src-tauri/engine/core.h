#ifndef CORE_H
#define CORE_H

#ifdef __cplusplus
extern "C" {
#endif

void generate_fractal(unsigned char* buffer, int width, int height, double zoom, double offset_x, double offset_y);

#ifdef __cplusplus
}
#endif

#endif