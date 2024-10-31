poly    START   0 .x⁴+2x³+3x²+4x+5 v točki x=2.
        LDX #in
        STX table_index
LOOP    LDX #a4
        STX koef_index
        LDB @table_index

IN_LOOP LDS @koef_index
        JSUB horner_step    .ai
        STA a
        LDA koef_index
        SUB #3
        RMO A,X
        STA koef_index
        LDA a
        LDT #a0
        COMPR X,T       
        JGT IN_LOOP
        . nalozi a0 in dodaj
        LDS @koef_index
        ADDR S,A
        STA @table_index

        LDX table_index
        RMO X,A
        ADD #3          . size of table iteration jump
        RMO A,X
        LDA #0
        STX table_index
        LDT #lastin
        COMPR X,T       
        JLT LOOP
halt    J halt

x       WORD    2
a0      WORD    5
a1      WORD    4
a2      WORD    3
a3      WORD    2
a4      WORD    1
table_index     RESW    1
koef_index     RESW    1
a       RESW 1
in      WORD 0
in2        WORD 5
in3       WORD 45
lastin  EQU *



horner_step ADDR S,A
            MULR B,A
            RSUB
