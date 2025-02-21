REC     START 0
        JSUB S_INIT
LOOP    CLEAR B
        JSUB READ
        COMP #0
        JEQ HALT
        LDB #1
        JSUB FACT 
        JSUB NUM
        JSUB NL
        J LOOP


HALT    J HALT


. READ FROM DEVICE FA
READ    TD	DEVICE
	JEQ	READ
        RD      DEVICE        . Preberemo znak iz vhoda
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
        LDS #10
        COMPR A,S
        JLT  DIGIT
        RMO A,T
L1      RMO T,A
        DIVR B,A        . UGOTOVI VELIKOST STEVILA
        COMPR A,S
        MULR X,B
        JGT  L1
        DIVR X,B
.        . prva stevka ze v A, V T JE CEL A
L2      ADD OFFSET
        WD #1
        SUB OFFSET
        RMO A,S
        RMO T,A
        STB L2_B
        MULR S,B
        SUBR B,A
        LDB L2_B
        DIVR X,B
        LDS #1
        COMPR B,S
        RMO A,T
        DIVR B,A
        JGT L2
DIGIT   ADD OFFSET
        WD #1
        RSUB

NL      STA NL_A
        LDA #10
        WD #1
        LDA NL_A
        RSUB
.__________FAKULTETA___________.
FACT    STL @SP         .PUSH L - SLED REKURZIJE
        JSUB PUSH
        STB @SP         .PUSH B
        JSUB PUSH

        COMP #1
        JEQ SUMOUT
        RMO A,B         .SHRANIMO A NEKAM
        SUB #1          .ZMANJSAMO A
        JSUB FACT       .POVOZIMO VREDNOST V L IN SE ZACIKLAMO
        MULR B,A
SUMOUT  JSUB POP      .POP V REGISTER B     .NAMESTO "LDL SUML"
        LDB @SP
        JSUB POP      .POP V REGISTER L, VNEMO SE V NALOV KJER SMO KLICALI JSUB FACT +1
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
DEVICE  BYTE 0xFA
        END REC