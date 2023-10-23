
; -------------
;  draw_line()
; -------------
; SP+4: PTR BANK (B)
; SP+2: PTR ADDR (W)
; SP: IMAGE WIDTH
; A: IMAGE HEIGHT
_draw_line:
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

umul16:
    sep #$10
    x8
    ldx T1
    stx $4202
    ldy T2
    sty $4203			;set up 1st multiply
    ldx T2+1
    clc
    lda $4216			;load $4216 for 1st multiply
    stx $4203			;start 2nd multiply
    sta T3
    stz T4			;high word of product needs to be cleared
    lda $4216			;read $4216 from 2nd multiply
    ldx T1+1
    stx $4202			;set up 3rd multiply
    sty $4203			;y still contains temp2
    ldy T2+1
    adc T3+1
    adc $4216			;add 3rd product
    sta T3+1
    sty $4203			;set up 4th multiply
    lda T4			;carry bit to last byte of product
    bcc :+
    adc #$00ff
:
    adc $4216			;add 4th product
    sta T4			;final store
    rep #$10
    x16
    rts

mul16:
    sep #$10
    x8
    ldx T1
    stx $4202
    ldy T2
    sty $4203			;set up 1st multiply
    ldx T2+1
    clc
    lda $4216			;load $4216 for 1st multiply
    stx $4203			;start 2nd multiply
    sta T3
    stz T4			;high word of product needs to be cleared
    lda $4216			;read $4216 from 2nd multiply
    ldx T1+1
    stx $4202			;set up 3rd multiply
    sty $4203			;y still contains temp2
    ldy T2+1
    adc T3+1
    adc $4216			;add 3rd product
    sta T3+1
    sty $4203			;set up 4th multiply
    lda T4			;carry bit to last byte of product
    bcc :+
    adc #$00ff
:
    adc $4216			;add 4th product
    cpx #$80
    bcc :+
    sbc T2
:
    cpy #$80
    bcc :+
    sbc T1
:
    sta T4			;final store
    rep #$10
    x16
    rts


div16:
    lda T1
    sta T3
    stz T4
    lda T2
    bne :+
    sec
    rts		;set carry to indicate divide by zero error
:
    cmp #$0100
    bcc divided_by_8_bit
:
    lsr                 ; Divide numerator by 2, 
    adc #$0000          ; adding carry flag into itself.
    lsr T3        ; Divide denominator by 2
    cmp #$0100          ; until it is under 256.
    bcs :-
    ldx T3
    stx $4204           ; WRDIVL
    sep #$20            ; 8-bit accumulator
    sta $4206           ; WRDIVB
    nop
    nop
    nop
    nop
    nop
    nop
    nop
    lda $4214           ; RDDIVL
    sta T4
    sta $4202           ; WRMPYA
    lda T2
    sta $4203           ; WRMPYB
    lda T2+1
    nop
    ldx $4216           ; RDMPYL
    sta $4203           ; WRMPYB
    stx T3
    lda T3+1
    clc
    adc $4216           ; RDMPYL
    sta T3+1
    rep #$20            ; 16-bit accumulator
    lda T1
    sec
    sbc T3
:
    cmp T2
    bcc :+
    sbc T2
    inc T4
    bra :-
:
    rts                     ;should end with carry clear to indicate valid answer
divided_by_8_bit:
    ldx T3
    stx $4204           ; WRDIVL
    sep #$20
    sta $4206           ; WRDIVB
    nop
    nop
    nop
    nop
    nop
    rep #$21		;clear carry to indicate valid answer
    lda $4214
    sta T4
    rts
