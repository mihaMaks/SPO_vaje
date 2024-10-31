REC     START 0
LOOP    CLEAR B
        JSUB READ
        COMP #0
        JEQ HALT
        JSUB S_INIT
        CLEAR B
        JSUB SUMN 
        JSUB NUM
        JSUB NL
        J LOOP


HALT    J HALT


. READ FROM DEVICE FA
READ    TD	    #250
	JEQ	    READ
        RD      #250        . Preberemo znak iz vhoda
        COMP    #10         . Preverimo, ali je konec datoteke
        JEQ     OUT         . Če je konec, končamo program
        COMP    #0
        JEQ     OUT
        STA     READ_A
        SUB     OFFSET
        ADDR    B,A
        RMO     A,B
        LDA     READ_A
        RMO     B,A
        LDA     READ_A
        RMO     B,A
        MUL     #10
        RMO     A,B
        CLEAR   A
        J       READ        . Gremo nazaj na branje
OUT     RMO B,A
        DIV #10
        RSUB
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

NL      STA NL_A
        LDA #10
        WD #1
        LDA NL_A
        RSUB
.__________FAKULTETA___________.
SUMN    STL @SP         .PUSH L - SLED REKURZIJE
        JSUB PUSH
        STB @SP         .PUSH B
        JSUB PUSH

        COMP #0
        JEQ SUMOUT
        RMO A,B         .SHRANIMO A NEKAM
        SUB #1          .ZMANJSAMO A
        JSUB SUMN       .POVOZIMO VREDNOST V L IN SE ZACIKLAMO
        ADDR B,A
SUMOUT  JSUB POP      .POP V REGISTER B     .NAMESTO "LDL SUML"
        LDB @SP
        JSUB POP      .POP V REGISTER L, VNEMO SE V NALOV KJER SMO KLICALI JSUB SUMN +1
        LDL @SP
        RSUB



._______________________.
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
OFFSET  WORD 48
L2_B    RESW 1
NL_A    RESW 1
READ_A  RESW 1
        END REC