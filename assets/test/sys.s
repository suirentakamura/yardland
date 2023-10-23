; ---------------------------------------------------------------------------
; sys.s
; ---------------------------------------------------------------------------
;
; System code for Yardland

	.setcpu	"65816"
    .smart on
    .importzp sp
    .import popa
    .export _mmu_map_bank
    .export _dma_transferb
    .export _DISPLAY
    .export _IMAGE

_DISPLAY = $A0000
_IMAGE = $10000

.segment "CODE"

; ----------------------------------------------------------------------
; void __fastcall__ mmu_map_bank (unsigned short real, unsigned char virt)
; ----------------------------------------------------------------------

.proc _mmu_map_bank

    sep #$20
    sta @Virt ; VIRT BANK (B)
    jsr popa
    sta @Real ; REAL BANK (W)
    jsr popa
    sta @Real + 1 ; REAL BANK (W)

    cop #2
    .byte 0

@Virt: .word $a5
@Real: .word $a5a5

    sep #$20

    rts

.endproc

; ----------------------------------------------------------------------------------------------
; void __fastcall__ dma_transferb (unsigned char *src, unsigned char *dest, unsigned long size, unsigned char type)
; ----------------------------------------------------------------------------------------------

.proc _dma_transferb

    sep #$20
    sta @Type
    rep #$20
    ldy #0
    lda (sp),y
    sta @Size
    iny
    iny
    lda (sp),y
    sta @Size + 2
    iny
    iny
    lda (sp),y
    sta @Dest
    iny
    iny
    lda (sp),y
    sta @Dest + 2
    iny
    iny
    lda (sp),y
    sta @Src
    iny
    iny
    lda (sp),y
    sta @Src + 2
    iny
    iny

    cop #6

@Type: .byte 1
@Src:  .dword $a5a5a5a5 ; Placeholder
@Dest: .dword $a5a5a5a5 ; Placeholder
@Size: .dword $a5a5a5a5 ; Placeholder

    lda sp
    sbc #12
    sta sp
    sep #$20

    rts

.endproc
