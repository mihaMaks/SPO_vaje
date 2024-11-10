graph   START 0
        LDA     #255
        JSUB    S_INIT
        . PARAMETRI ZA VERTICAL LINE X=IZHODISCE T=DOLZINA
        LDX     SCREEN
        LDB     WIDTH
        LDT     L_SIZE
        MULR    T,B
        ADDR    B,X
        ADDR    T,X

        
        LDA #210
        JSUB    H
        LDA #205
        JSUB    A
        LDA #252
        JSUB    P
        LDA #191
        JSUB    P
        LDA #143
        JSUB    Y
        JSUB NEW_L
        LDA #189
        JSUB    B
        LDA #183
        JSUB    I
        LDA #248
        JSUB    R
        LDA #172
        JSUB    T
        LDA #207
        JSUB    H
        JSUB NEW_L
        LDA #240
        JSUB    D
        LDA #237
        JSUB    A
        LDA #249
        JSUB    Y


HALT    J   HALT
..................................................
..... IN REGISTER T WE STORE LENGTH OF LINE ......
..... IN T WE STORE WHERE LINE ENDS (INSIDE METHODS)
..................................................
VERT_2  LDB     WIDTH
        MULR    B,T
        ADDR    X,T
VER_L2  STCH    0,X
        STCH    1,X
        ADDR    B,X
        COMPR   X,T
        JLT     VER_L2
        SUBR    B,X
        RSUB
VERT_1  LDB     WIDTH
        MULR    B,T
        ADDR    X,T
VER_L1  STCH    0,X
        ADDR    B,X
        COMPR   X,T
        JLT     VER_L1
        SUBR    B,X
        RSUB

DIAG_D  LDB     WIDTH
        RMO     T,S     .KER GREMO DIAGONALNO RABMO DODAT TUDI ZACETNI T
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    B,T
        ADDR    X,T
        ADDR    S,T
        LDS     #1
DID_L   STCH    0,X
        STCH    1,X
        ADDR    B,X
        STCH    0,X
        STCH    1,X
        ADDR    B,X
        ADDR    S,X
        COMPR   X,T
        JLT     DID_L
        SUBR    B,X
        SUBR    S,X
        RSUB

DIAG1D  LDB     WIDTH
        RMO     T,S     .KER GREMO DIAGONALNO RABMO DODAT TUDI ZACETNI T
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    B,T
        ADDR    X,T
        ADDR    S,T
        LDS     #1
DID1L   STCH    0,X
        ADDR    B,X
        ADDR    S,X
        COMPR   X,T
        JLT     DID1L
        SUBR    B,X
        SUBR    S,X
        RSUB

DIAG_R  LDB     WIDTH
        RMO     T,S     .KER GREMO DIAGONALNO RABMO DODAT TUDI ZACETNI T
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    B,T
        ADDR    X,T
        ADDR    S,T
        LDS     #1
DIR_L   STCH    0,X
        STCH    64,X
        ADDR    B,X
        ADDR    S,X
        COMPR   X,T
        JLT     DIR_L
        SUBR    B,X
        SUBR    S,X
        RSUB


DIAG_U  RMO     T,S     .KER GREMO DIAGONALNO RABMO DODAT TUDI ZACETNI T
        LDB     WIDTH
        MULR    B,T
        STX     TEMP 
        SUBR    T,X
        RMO     X,T
        LDX     TEMP
        ADDR    S,T
        LDS     #1
DIU_L   STCH    0,X
        STCH    1,X
        SUBR    B,X
        STCH    0,X
        STCH    1,X
        SUBR    B,X
        ADDR    S,X
        COMPR   X,T
        JGT     DIU_L
        RSUB

DIAG1U  RMO     T,S     .KER GREMO DIAGONALNO RABMO DODAT TUDI ZACETNI T
        LDB     WIDTH
        MULR    B,T
        STX     TEMP 
        SUBR    T,X
        RMO     X,T
        LDX     TEMP
        ADDR    S,T
        LDS     #1
DIU1L   STCH    0,X
        SUBR    B,X
        ADDR    S,X
        COMPR   X,T
        JGT     DIU1L
        RSUB

HOR_2   ADDR    X,T
LP_2    STCH    0,X
        STCH    64,X
        TIXR    T
        JLT     LP_2
        LDB     #1
        SUBR    B,X
        RSUB

HOR_1   ADDR    X,T
LP_1    STCH    0,X
        TIXR    T
        JLT     LP_1
        LDB     #1
        SUBR    B,X
        RSUB

.******************************
.*          LETTERS           *
.******************************

H       STL     @SP
        JSUB    PUSH

        LDT     L_SIZE
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        SUBR    B,X

        STX     TEMP
        LDT     L_SIZE
        JSUB    VERT_2
        LDT     L_SIZE          . PREMAKNEMO SE NA SREDINO CRKE
        LDB     #2
        DIVR    B,T
        LDB     WIDTH
        MULR    T,B             . B = L_SIZE/2 * WIDTH
        SUBR    B,X
        JSUB    HOR_2
        LDB     L_SIZE         
        LDT     #2
        DIVR    T,B            . B = L_SIZE/2
        LDX     TEMP
        ADDR    B,X
        LDT     L_SIZE
        JSUB    VERT_2
        
        LDB     #3
        ADDR    B,X

        JSUB    POP
        LDL     @SP
        RSUB

A       STL     @SP
        JSUB    PUSH
        STX     TEMP
        LDT     L_SIZE
        JSUB    DIAG_U
        LDB     WIDTH
        ADDR    B,X
        LDT     L_SIZE
        JSUB    DIAG_D
        LDX     TEMP
        LDB     #2
        LDT     L_SIZE
        DIVR    B,T
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        SUBR    B,X
        LDT     L_SIZE
        LDB     #4
        DIVR    B,T
        ADDR    T,X
        RMO     T,S
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        JSUB    HOR_1

        LDX     TEMP
        LDT     L_SIZE
        ADDR    T,X
        ADDR    B,X
        TIXR    T
        
        JSUB    POP
        LDL     @SP
        RSUB
        
P       STL     @SP
        JSUB    PUSH
        
        LDT     L_SIZE
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        SUBR    B,X

        STX     TEMP
        LDT     L_SIZE
        JSUB    VERT_2
        LDX     TEMP
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        JSUB    HOR_1
        LDB     #1
        ADDR    B,X
        LDB     WIDTH
        ADDR    B,X
        LDB     #2
        LDT     L_SIZE
        DIVR    B,T             . T = L_SIZE / 2
        LDB     #1
        SUBR    B,T
        JSUB    VERT_1
        LDX     TEMP
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        LDB     WIDTH
        MULR    T,B
        ADDR    B,X
        JSUB    HOR_1
        LDX     TEMP
        LDB     #1
        LDT     L_SIZE
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        STCH    0,X
        ADDR    B,X
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        ADDR    B,X
        ADDR    T,X
        

        JSUB    POP
        LDL     @SP
        RSUB

Y       STL     @SP
        JSUB    PUSH
        
        LDT     L_SIZE
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        SUBR    B,X

        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        JSUB    DIAG1D
        STX     TEMP
        TIXR    T
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        JSUB    DIAG1U

        LDX     TEMP
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        LDB     #1
        SUBR    B,X
        ADDR    B,T
        JSUB    VERT_2

        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        ADDR    T,X
        ADDR    B,X
        
        JSUB    POP
        LDL     @SP
        RSUB

B       STL     @SP
        JSUB    PUSH
        
        LDT     L_SIZE
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        SUBR    B,X

        STX     TEMP
        LDT     L_SIZE
        JSUB    VERT_2
        LDX     TEMP
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        JSUB    HOR_1
        LDB     #1
        ADDR    B,X
        LDB     WIDTH
        ADDR    B,X
        LDB     #2
        LDT     L_SIZE
        DIVR    B,T             . T = L_SIZE / 2
        LDB     #1
        SUBR    B,T
        JSUB    VERT_1
        LDX     TEMP
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        LDB     WIDTH
        MULR    T,B
        ADDR    B,X
        STX     @SP
        JSUB    PUSH
        JSUB    HOR_1
        .. NASTAVIMO X NA PRAVO MESTO
        LDT     L_SIZE
        LDB     #2
        TIXR    T
        DIVR    B,T
        SUBR    T,X

        .. PONOVIMO POLKROG
        JSUB    HOR_1
        LDB     #1
        ADDR    B,X
        LDB     WIDTH
        ADDR    B,X
        LDB     #2
        LDT     L_SIZE
        DIVR    B,T             . T = L_SIZE / 2
        LDB     #1
        SUBR    B,T
        JSUB    VERT_1
        JSUB    POP
        LDX     @SP
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        ADDR    B,X
        LDB     #1
        ADDR    B,T
        JSUB    HOR_1

        LDB     #3
        ADDR    B,X

        JSUB    POP
        LDL     @SP
        RSUB

R       STL     @SP
        JSUB    PUSH
        
        LDT     L_SIZE
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        SUBR    B,X

        STX     TEMP
        LDT     L_SIZE
        
        JSUB    VERT_2
        LDX     TEMP
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        JSUB    HOR_1
        LDB     #1
        ADDR    B,X
        LDB     WIDTH
        ADDR    B,X
        LDB     #2
        LDT     L_SIZE
        DIVR    B,T             . T = L_SIZE / 2
        LDB     #1
        SUBR    B,T
        JSUB    VERT_1
        LDX     TEMP
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        LDB     WIDTH
        MULR    T,B
        ADDR    B,X
        STX     TEMP
        JSUB    HOR_1

        LDX     TEMP
        TIXR    T
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        LDB     #1
        SUBR    B,T
        JSUB    DIAG_R
        LDB     WIDTH
        ADDR    B,X
        LDB     #3
        ADDR    B,X
        
        JSUB    POP
        LDL     @SP
        RSUB

T       STL     @SP
        JSUB    PUSH
        
        LDT     L_SIZE
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        SUBR    B,X

        LDT     L_SIZE
        JSUB    HOR_2
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        SUBR    T,X
        LDT     L_SIZE
        JSUB    VERT_2
        
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        ADDR    T,X
        ADDR    B,X

        JSUB    POP
        LDL     @SP
        RSUB

D       STL     @SP
        JSUB    PUSH
        
        LDT     L_SIZE
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        SUBR    B,X

        STX     TEMP
        LDT     L_SIZE
        JSUB    VERT_2
        LDX     TEMP
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        JSUB    HOR_1
        LDB     #1
        ADDR    B,X
        LDB     #4
        LDT     L_SIZE
        DIVR    B,T             . T = L_SIZE / 4
        JSUB    DIAG1D
        
        LDT     L_SIZE
        LDB     #4
        DIVR    B,T             . T = L_SIZE / 4
        LDB     L_SIZE
        SUBR    T,B
        RMO     B,T
        JSUB    VERT_1
        
        LDX     TEMP
        LDT     L_SIZE
        LDB     WIDTH
        MULR    T,B
        ADDR    B,X
        LDT     L_SIZE
        LDB     #2
        DIVR    B,T
        JSUB    HOR_1
        
        LDB     #1
        ADDR    B,X
        LDB     #4
        LDT     L_SIZE
        DIVR    B,T             . T = L_SIZE / 4
        JSUB    DIAG1U

        LDB     #4
        LDT     L_SIZE
        DIVR    B,T
        LDB     WIDTH
        MULR    T,B
        ADDR    B,X
        TIXR    T

        JSUB    POP
        LDL     @SP
        RSUB

I       STL     @SP
        JSUB    PUSH

        LDT     L_SIZE
        LDB     #1
        SUBR    B,T
        LDB     WIDTH
        MULR    T,B
        SUBR    B,X

        STX     TEMP
        LDT     L_SIZE
        JSUB    VERT_2
        
        LDB     #3
        ADDR    B,X

        JSUB    POP
        LDL     @SP
        RSUB

NEW_L   LDB     #64
        LDS     SCREEN
        SUBR    S,X
        DIVR    B,X
        LDB     L_SIZE
        ADDR    B,X
        LDB     #3
        ADDR    B,X     
        LDB     WIDTH
        MULR    B,X
        ADDR    S,X
        LDB     L_SIZE
        ADDR    B,X
        RSUB


. NASTAVI VREDNOST SP NA ZACETEK STACKA
S_INIT  STA STACKA
        LDA #STACK
        STA #SP
        LDA STACKA
        RSUB
. POVECA VREDNOST SP ZA ENO BESEDO
PUSH    STA STACKA
        LDA SP
        ADD #3
        STA SP
        LDA STACKA
        RSUB
. ZMANJSA VREDNOST SP ZA ENO BESEDO
POP     STA STACKA
        LDA SP
        SUB #3
        STA SP
        LDA STACKA
        RSUB

SP      WORD 0                  . STACK POINTER
STACKA  WORD 0
STACK   RESW 1000

SCREEN  WORD X'00A000'
WIDTH   WORD X'000040'
L_SIZE  WORD X'00000A' . VELIKOST CRK NAJ BO SODA
TEMP    RESW 1
        END graph