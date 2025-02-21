ECHO    START 0
        LDA #NIZ       . TEST STRING SUBROUTINE
        CLEAR X
        JSUB STRING   

        JSUB NL        . TEST NL SUBROUTINE

        LDA STEVILO     . NUM SUBROUTINE
        JSUB NUM
        JSUB NL


HALT    J HALT
. izpis znaka, podanega v registru A.
CHAR    WD #1
        RSUB
.izpis znaka za skok v novo vrstico.
NL      STA NL_A
        LDA #10
        WD #1
        LDA NL_A
        RSUB
.zpis niza, ki se nahaja na naslovu, podanem v registru A. 
.Podprite C-jevske nize, t.j. nizi se končajo z znakom s kodo 0. 
STRING  RMO A,X
        LDCH 0, X
        LDT #0
        WD #1
        COMPR A,T
        RMO X,A
        ADD #1
        JGT STRING
        RSUB
.desetiški izpis števila, podanega v registru A.
.Ta rutina je malce bolj zapletena, zato je pametno malce vnaprej razmisliti.
.Kako pridemo do števk števila? V kakšnem vrstnem redu jih dobimo?
NUM     LDB #1
        LDX #10
        RMO A,T
L1      DIVR B,A        . UGOTOVI VELIKOST STEVILA
        LDS #0
        COMPR A,S
        MULR X,B
        RMO T,A
        JGT  L1
        DIVR X,B        . NASTAVI B NA FLOOR LOG10(STEVILO)
        DIVR X,B
L2      DIVR B,A        . N-TA STEVKA
        ADD OFFSET
        WD #1
        SUB OFFSET
        RMO A,S
        RMO T,A
        STB L2_B
        MULR S,B
        SUBR B,A
        LDB L2_B
        DIVR X,B
        LDS #0
        COMPR B,S
        RMO A,T
        JGT L2
        RSUB

NL_A    RESW 1
L2_B    RESW 1
OFFSET  WORD 48
STEVILO WORD 34267
NIZ     BYTE C'TO JE NIZ KI SE KONCA Z KODO 0'
        BYTE 0
END     EQU *   
        END ECHO