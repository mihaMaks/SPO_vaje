prog    START 0
        SHIFTL A,10
        ADDR S,T
        MULR B , X
        CLEAR F
        SHIFTR B,12
        END prog

x       WORD    12
y       WORD    5
sum     RESW    1
diff    RESW    1
prod    RESW    1
quot    RESW    1
mod     RESW    1