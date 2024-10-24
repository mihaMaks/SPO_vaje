poly    START   0 .x⁴+2x³+3x²+4x+5 v točki x=2.
        LDT #lastin
        LDX #in
        STX table_index
LOOP    LDB @table_index
        LDS a4
        JSUB horner_step    .a4
        LDS a3
        JSUB horner_step    .a3
        LDS a2
        JSUB horner_step    .a2
        LDS a1
        JSUB horner_step    .a1
        ADD a0
        STA @table_index
        RMO X,A
        ADD #3          . size of table iteration jump
        RMO A,X
        LDA #0
        STX table_index
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
in      WORD 0
        WORD 5
        WORD 45
lastin  EQU *



horner_step ADDR S,A
            MULR B,A
            RSUB
