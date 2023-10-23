#include <stdbool.h>
#include "sys.h"

#define DISP_W 1024
#define DISP_H 720
#define DISPLAY ((unsigned char*)0xA0000)
#define IMAGE ((unsigned char*)0x10000)

/*
void draw_image(int w, int h, unsigned *src) {
    int line = 0, pix = 0;

    for (; line < (h * 2); line++) {
        for (; pix < (w * 2); pix++) {
            DISP[pix] = src[pix];
        }
    }

    return;
}
*/

void main() {
    unsigned short int y = 0;

    //dma_transferb(0x10000, 0xA0000, 0x280, DMA_TRANSFERB_VR);
    //dma_transferb(0x10280, 0xA0800, 0x280, DMA_TRANSFERB_VR);

    while (true) {
        unsigned short int s_off = y * 640;
        unsigned short int d_off = y * 2048;
        dma_transferb(0x1, 0x0 + s_off, 0xA, 0x0 + d_off, (unsigned long int)640, DMA_TRANSFERB_VR);
        ++y;
    }

    return;
}
