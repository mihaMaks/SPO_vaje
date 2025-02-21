BALL   START 0
        LDT LIMIT
        LDX SCREEN
        LDA #0x0            . OFFSET BALL START
        ADDR A,X    
        LDA #0x4F
        STCH 0,X            . PRINT BALL 
LOOP    STX STORAGE_X

.        . move ball
        STX STORAGE_X
        JSUB MOVE_XY
        JSUB CHECK_R        . CHECK FOR COLLISION
BACK_R  JSUB CHECK_DOWN     
BACK_D  JSUB CHECK_UP
        LDS #1
        LDB OOB
        COMPR S,B          . CHECK IF COLLISION IS DETECTED
        JGT NOT_OOB        . IF OUT OF BOUNDS RETURN TO PREVIOUS POSITION AND MOVE FROM THERE
        LDX STORAGE_X
        JSUB MOVE_XY
        LDS #0
        STS OOB
NOT_OOB LDA STORAGE_A
        STCH 0,X            . PRINT BALL
        
        RMO A,S             . ERASE BALL
        LDA #0
        STX STORAGE_X2      . STORE CURRENT X
        LDX STORAGE_X       . LOAD PREVIOUS X
        STCH 0,X            . 
        LDX STORAGE_X2      . LOAD CURRENT X
        RMO S,A
        J LOOP

. VECTOR OF BALL MOVEMENT
DX      WORD X'000001'
DY      WORD X'000001'
ST_DY   WORD X'000000'
ST_DX   WORD X'000000'
STORAGE_A RESW 1
STORAGE_X RESW 1
STORAGE_X2 WORD X'00B800'

. Default data for screen
SCREEN  WORD    X'00B800'
WIDTH   BYTE    X'000050'
HEIGHT  BYTE    X'000019'
. 
LIMIT   WORD    X'00BFCF'
MINUS   WORD    X'FFFFFF' 
OOB     WORD    X'000000' . OUT OF BOUNDS FLAG
.
CHECK_DOWN  LDS LIMIT
            COMPR X,S
            JGT BOUNCE_DY
            RSUB
CHECK_UP    LDS SCREEN
            COMPR X,S
            JLT BOUNCE_DY
            RSUB
BOUNCE_DY   LDS DY
            LDB DY
            SUBR S,B
            SUBR S,B
            STB DY
            LDS #1
            STS OOB
            RSUB
CHECK_R     STA STORAGE_A
            LDA STORAGE_X
            RMO X,B
            LDS SCREEN
            LDA #240
            SUBR A,S
            LDA STORAGE_X
            SUBR S,A
            SUBR S,B
            LDS WIDTH
            DIVR S,A
            DIVR S,B
            SUBR A,B
            LDA DY
            COMPR A,B
            LDA STORAGE_A
            JEQ BACK_R
            J BOUNCE_DX
BOUNCE_DX   LDS DX
            LDB DX
            SUBR S,B
            SUBR S,B
            STB DX
            LDS #1
            STS OOB
            J BACK_R
MOVE_XY     STA STORAGE_A
            LDA DY
            MUL WIDTH
            LDB DX
            ADDR B,A
            ADDR A,X
            LDA STORAGE_A
            RSUB
        END     BALL