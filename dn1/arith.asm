prog    START 0
. neposredno naslavljanje
.        . sestevanje
        LDA x
        ADD y
        STA sum
.        . odstevanje
        LDA x
        SUB y
        STA diff
.        . mnozenje
        LDA x
        MUL y
        STA prod
.        . kvocient
        LDA x
        DIV y
        STA quot
.        . mod
        LDA y
        MUL quot
        STA mod
        LDA x
        SUB mod
        STA mod
        
halt    J halt


x       WORD    12
y       WORD    5
sum     RESW    1
diff    RESW    1
prod    RESW    1
quot    RESW    1
mod     RESW    1