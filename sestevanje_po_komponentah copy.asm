.program sesteje ALFA+BETA in rezultat zapise v GAMA

ADDLP   LDX     INDEX
        LDA     ALFA,X
        ADD     BETA,X
        STA     GAMA,X
        LDA     INDEX
        ADD     TRI
        STA     INDEX
        COMP    DEVET
        JLT     ADDLP
HALT    J       HALT



TRI     WORD    3
INDEX   WORD    0
ALFA    WORD    5
        WORD    7
        WORD    3
BETA    WORD    1
        WORD    2
        WORD    3
GAMA    RESW    3