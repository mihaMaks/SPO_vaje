poly    START   0 .x⁴+2x³+3x²+4x+5 v točki x=2.
        LDA x
        JSUB square
        JSUB square     .A=x^4
        RMO A,B         .B=x^4

        LDA x           .reset A
        JSUB cube       .A=x^3
        MUL a3          .A=a3*x^3
        ADDR A,B        .B= x^4 + a3*x^3

        LDA x           .reset A
        JSUB square
        MUL a2
        ADDR A,B        .B= x^4 + a3*x^3 + a2*x^2
        
        LDA x           .reset A
        MUL a1
        ADDR A,B        .B= x^4 + a3*x^3 + a2*x^2 + a1*x
        RMO B,A
        ADD a0
        STA res
halt    J halt

x       WORD    2
a0      WORD    5
a1      WORD    4
a2      WORD    3
a3      WORD    2
cube_b  RESW    1
res     RESW    1

square  MULR A,A
        RSUB

cube    STB cube_b
        RMO A,B
        MULR A,A
        MULR B,A
        LDB cube_b
        RSUB
        