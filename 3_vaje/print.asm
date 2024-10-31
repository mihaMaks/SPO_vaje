PRINT    START 0
        CLEAR X
LOOP    LDCH TXT,X
        JSUB PUTC               .STDOUT
        TIX #LEN
        JLT LOOP
        JSUB NL
HALT    J HALT

. subrutina ki izpise znak v registru A
PUTC    WD 0xAA
        RSUB
NL      STA NL_A
        LDCH #10
        WD 0xAA
        LDA NL_A
        RSUB
TXT     BYTE   C'SIC/XE'
END     EQU *
LEN     EQU END - TXT           .LEN NI NASLOV V RAMU AMPAK NASLOV END - TXT = DOLZINA TABELE
NL_A    RESW 1
DEVICE WORD X'000001'
        END PRINT