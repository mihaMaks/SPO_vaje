. to je komentar
prog1   START 0
        LDA x
        LDB #9
        MULR B, A
        STA out
halt    J halt

. spodaj so podatki
x       WORD 6
out     RESW 1
        END prog1
