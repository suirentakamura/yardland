    include "zp.asm"

    org $E000

DISP_W = 1024
DISP_H = 720

RESET:
    CLC
    XCE
    SEI
    REP #$30
    a16
    x16

    LDA #320 ; Image width
    STA T1
    LDA #240 ; Image Height
    STA T2
    LDA #<IMAGE ; load Image addr
    STA T3
    SEP #$20
    a8
    LDA #^IMAGE ; load Image bank
    STA T3 + 2
    REP #$20
    a16
    JSR DRAW_IMAGE
    STP
    BRA	RESET

; T1 = IMAGE WIDTH
; T2 = IMAGE HEIGHT
; T3 = PTR
DRAW_IMAGE:
    LDA T3
    STA .SRC ; Save T3 Addr
    SEP #$20
    a8
    LDA T3 + 2
    PHA ; Save T3 Bank
    REP #$20
    a16
    JSR umul16 ; Operands T1 and T2, Result in T3/T4 (H/L)
    LDA T4
    ASL
    ;STA SIZE
    LDA T3
    ROL
    ;STA SIZE + 2
    SEP #$20
    a8
    PLA ; Restore T3 Bank
    STA .SRC + 2
    REP #$20
    a16
    PLA
    STA .SRC

    COP #6 ; Inst size (6 words)
        db 1   ; Opcode = MMU DMA TRANSFERB VR

.SRC:   dw 0     ; SRC Addr
        dw 0     ; SRC Virtual Bank
.DEST:  dw 0     ; DEST Addr
        dw $0A   ; DEST Real Bank
.SIZE:  dw $5800 ; SIZE
        dw 2

    RTS

    org $F000

    include "lib.asm"
    include "drv.asm"

;===============================================================================
; Dummy Interrupt Handlers
;-------------------------------------------------------------------------------

NMIN:
NMI:
IRQBRK:
IRQN:
BRKN:
COPN:
COP:
    STP

;===============================================================================
; Vectors
;-------------------------------------------------------------------------------

    org $FFE0

    dw 0    ; Reserved
    dw 0    ; Reserved
    dw COPN                    ; $FFE4 - COP(816)
    dw BRKN                    ; $FFE6 - BRK(816)
    dw 0                       ; $FFE8 - ABORT(816)
    dw NMIN                    ; $FFEA - NMI(816)
    dw 0                       ; Reserved
    dw IRQN                    ; $FFEE - IRQ(816)

    dw 0
    dw 0
    dw COP                     ; $FFF4 - COP(C02)
    dw 0                       ; Reserved
    dw 0                       ; $FFF8 - ABORT(C02)
    dw NMI                     ; $FFFA - NMI(C02)
    dw RESET                   ; $FFFC - RESET(C02)
    dw IRQBRK                  ; $FFFE - IRQBRK(C02)

IMAGE = $10000
