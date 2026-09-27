#include "core.h"

void generate_fractal(unsigned char* buffer, int width, int height, double zoom, double offset_x, double offset_y) {
    int max_iter = 256;

    for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
            double cx = (x - width / 2.0) * zoom + offset_x;
            double cy = (y - height / 2.0) * zoom + offset_y;

            double zx = 0.0;
            double zy = 0.0;

            int iter = 0;
            while ((zx * zx + zy * zy) < 4.0 && iter < max_iter) {
                double temp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = temp;
                iter++;
            }

            int pixel_index = (y * width + x) * 4;
            unsigned char color = (iter == max_iter) ? 0 : (unsigned char)(iter * 255 / max_iter);

            buffer[pixel_index] = color;
            buffer[pixel_index + 1] = color;
            buffer[pixel_index + 2] = color;
            buffer[pixel_index + 3] = 255;
        }
    }
}