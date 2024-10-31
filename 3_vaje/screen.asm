SCR  START 0
        LDA #48
        JSUB SCRFILL
        JSUB SCRCLEAR
HALT    J HALT

SCRCLEAR        STA ST_A
                CLEAR A
                LDX SCREEN
LOOP            STCH 0,X
                TIX LIMIT
                JLT LOOP
                LDA ST_A
                RSUB
SCRFILL         LDX SCREEN
LOOP2           STCH 0,X
                TIX LIMIT
                JLT LOOP2
                RSUB

. Default data for screen
SCREEN  WORD    X'00B800'
SCRCOLS BYTE    X'000050'
SCRROWS BYTE    X'000019'
SCRLEN  WORD    X'0007CF'
LIMIT   WORD    X'00BFD0'
ST_A    RESW    1
        END     SCR
