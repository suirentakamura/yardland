; ----------------
; | MMU MAP BANK |
; ----------------
; SP: REAL BANK (W)
; A: VIRT BANK (B)
MMU_MAP_BANK:
    SEP #$20
    a8
    STA .VIRT ; VIRT BANK (B)
    REP #$20
    a16
    PLA
    STA .REAL ; REAL BANK (W)

    COP #3
    db 0

.VIRT: dw $a5
.REAL: dw $a5a5

    RTS

; ---------------------
; | DMA TRANSFER B VR |
; ---------------------
; SP+5: SIZE (2W)
; SP+1: DEST REAL (2W)
; SP: SRC VIRTUAL BANK (B)
; A: SRC ADDR (W)
DMA_TRANSFER_B_VR:
    STA .SRC ; SRC ADDR (W)
    SEP #$20
    a8
    PLA
    STA .SRC + 2 ; SRC VIRTUAL BANK (B)
    REP #$20
    a16
    PLA
    STA .DEST
    PLA
    STA .DEST + 2
    PLA
    STA .SIZE
    PLA
    STA .SIZE + 2

    COP #6
    db 1

.SRC:  defl $a5a5a5a5 ; Placeholder
.DEST: defl $a5a5a5a5 ; Placeholder
.SIZE: defl $a5a5a5a5 ; Placeholder
    
    RTS

; --------------------
; | DMA TRANSFER B V |
; --------------------
; SP+5: SIZE (2W)
; SP+1: DEST VIRTUAL (2W)
; SP: SRC VIRTUAL BANK (B)
; A: SRC ADDR (W)
DMA_TRANSFER_B_V:
    STA .SRC ; SRC ADDR (W)
    SEP #$20
    a8
    PLA
    STA .SRC + 2 ; SRC VIRTUAL BANK (B)
    REP #$20
    a16
    PLA
    STA .DEST
    PLA
    STA .DEST + 2
    PLA
    STA .SIZE
    PLA
    STA .SIZE + 2

    COP #6
    db 2

.SRC:  defl $a5a5a5a5 ; Placeholder
.DEST: defl $a5a5a5a5 ; Placeholder
.SIZE: defl $a5a5a5a5 ; Placeholder
    
    RTS

; ---------------------
; | DMA TRANSFER B R |
; ---------------------
; SP+5: SIZE (2W)
; SP+1: DEST REAL (2W)
; SP: SRC REAL BANK (B)
; A: SRC REAL ADDR (W)
DMA_TRANSFER_B_R:
    STA .SRC ; SRC ADDR (W)
    SEP #$20
    a8
    PLA
    STA .SRC + 2 ; SRC VIRTUAL BANK (B)
    REP #$20
    a16
    PLA
    STA .DEST
    PLA
    STA .DEST + 2
    PLA
    STA .SIZE
    PLA
    STA .SIZE + 2

    COP #6
    db 3

.SRC:  defl $a5a5a5a5 ; Placeholder
.DEST: defl $a5a5a5a5 ; Placeholder
.SIZE: defl $a5a5a5a5 ; Placeholder
    
    RTS
