PUTS    START 0
        CLEAR X
LOOP    LDCH TXT,X
        JSUB PUTC               .STDOUT
        TIX #LEN
        JLT LOOP
HALT    J HALT

. subrutina ki izpise znak v registru A
PUTC    WD #250
        RSUB

TXT     BYTE   C'123'
        BYTE X'00000A'
END     EQU *
LEN     EQU END - TXT           .LEN NI NASLOV V RAMU AMPAK NASLOV END - TXT = DOLZINA TABELE
        END PUTS