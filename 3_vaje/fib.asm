FIB     START 0
        
        
        
        
        
.   SUBRUTINE        
. BRANJE IZ NAPRAVE FA

. FIBBONACHI
FIB     STL @SP         .PUSH L - SLED REKURZIJE
        JSUB PUSH
        STB @SP         .PUSH B
        JSUB PUSH

        COMP #0
        JEQ FIB2
        COMP #1
        JEQ FIB2
        
        RMO A,B
        SUB #1
        JSUB FIB        . FIB(N-1)
FIB2    ADDR A,S
        JSUB POP
        LDA @SP

        COMP #0
        JEQ FIBOUT
        COMP #1
        JEQ FIBOUT
        
        RMO A,B
        SUB #2
        JSUB FIB





.       STACK
. VZOREC POP:
.       JSUB POP        PREMIK SP V -
.       LDB @SP         NALGANJE IZ SP V POLJUBEN REGISTER
. VZOREC PUSH:
.       STB @SP         LANAGANJE NA STACK  
.       JSUB PUSH       PREMIK SP V +
.
.
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
        END FIB