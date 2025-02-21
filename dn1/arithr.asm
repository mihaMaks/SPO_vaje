prog    START 0
        LDA y
        LDB x
.        . sestevanje
        ADDR B,A
        STA sum
        SUBR B,A
.        . odstevanje
        SUBR A,B
        STB diff
        ADDR A,B
.        . mnozenje
        MULR B,A
        STA prod
        DIVR B,A
.        . kvocient
        RMO B,X     .shranimo B za mod
        DIVR A,B
        STB quot
.        . mod
        MULR B,A
        SUBR A,X
        STX mod
halt    J halt

x       WORD    11 
y       WORD    4
sum     RESW    1
diff    RESW    1
prod    RESW    1
quot    RESW    1
mod     RESW    1

