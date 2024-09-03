/**
 * sys.h
 * 
 * System code for Yardland
*/

#define DMA_TRANSFERB_VR 1
#define DMA_TRANSFERB_V  2
#define DMA_TRANSFERB_R  3

#define lptrt unsigned long int
#define lptr(X) ((lptrt)X)

extern void mmu_map_bank(unsigned short real, unsigned char virt);
extern void dma_transferb(lptrt src, lptrt dest, unsigned long int size, unsigned char type);
//extern void dma_transferb(unsigned short src_h, unsigned short src_l, unsigned short dest_h, unsigned short dest_l, unsigned long size, unsigned char type);
