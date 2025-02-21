PRINT    START 0
        CLEAR X
LOOP    LDCH TXT,X
        JSUB PUTC               .STDOUT
        TIX #LEN
        JLT LOOP
        JSUB NL
HALT    J HALT

. subrutina ki izpise znak v registru A
PUTC    WD #170
        RSUB
NL      STA NL_A
        LDCH #10
        WD #1
        LDA NL_A
        RSUB
TXT     BYTE   C'SIC/XE!WA'
END     EQU *
LEN     EQU END - TXT           .LEN NI NASLOV V RAMU AMPAK NASLOV END - TXT = DOLZINA TABELE
NL_A    RESW 1
DEVICE  WORD 170
        END PRINT