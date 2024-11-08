PROG    START 0
        JSUB S_INIT
        LDA #1
        LDB #2

        STA @SP                 . DAMO NA STACK
        JSUB PUSH
        STB @SP
        JSUB PUSH

        JSUB POP              . VZAMEMO DOL
        LDA @SP
        JSUB POP
        LDB @SP

HALT    J HALT

. NASTAVI VREDNOST SP NA ZACETEK STACKA
S_INIT  STA STACKA
        LDA #STACK
        STA #SP
        LDA STACKA
        RSUB
. POVECA VREDNOST SP ZA ENO BESEDO
PUSH  STA STACKA
        LDA SP
        ADD #3
        STA SP
        LDA STACKA
        RSUB
. ZMANJSA VREDNOST SP ZA ENO BESEDO
POP   STA STACKA
        LDA SP
        SUB #3
        STA SP
        LDA STACKA
        RSUB


SP      WORD 0                  . STACK POINTER
STACKA  WORD 0
STACK   RESW 1000
        END PROG