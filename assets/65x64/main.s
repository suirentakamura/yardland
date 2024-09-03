.org $0000_0000_0000_0000

.org $0000_0000_0000_e000

; Clear memory
clrmem:
    lda #0           ; Set up zero value
:   dey              ; Decrement counter
    sta (sp[0]),Y    ; Clear memory location
    bne :-           ; Not zero, continue checking
    rts              ; Return
