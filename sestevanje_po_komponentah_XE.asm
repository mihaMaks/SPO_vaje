.program sesteje ALFA+BETA in rezultat zapise v GAMA

        LDS     #3
        LDT     #9
        CLEAR   X
ADDLP   LDA     ALFA,X
        ADD     BETA,X
        STA     GAMA,X
        ADDR    S,X
        COMPR   X,T
        JLT     ADDLP




ALFA    WORD    5
        WORD    7
        WORD    3
BETA    WORD    1
        WORD    2
        WORD    3
GAMA    RESW    3

.DN - napisi porgram odbijajoce zogice