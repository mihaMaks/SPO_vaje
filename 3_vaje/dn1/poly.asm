poly    START   0 .x⁴+2x³+3x²+4x+5 v točki x=2.
        LDB x
        MULR B,B        
        MULR B,B        .B=x4
        LDA x
        MULR A,A        
        MUL x           .A=x^3
        MUL a3          .A=2*x^3
        ADDR A,B        .B=x^4 + 2*x^3
        LDA x
        MULR A,A        .A=x^2
        MUL a2          .A=3*x^2
        ADDR A,B        .B=x^4+2*x^3 + 3*x^2
        LDA x
        MUL a1          .A=x*4
        ADD a0          .A=x*4+5
        ADDR B,A        .A= x^4+2*x^3+3*x^2 + x*4+5
        STA res
halt    J halt

x       WORD    2
a0      WORD    5
a1      WORD    4
a2      WORD    3
a3      WORD    2
res     RESW    1

