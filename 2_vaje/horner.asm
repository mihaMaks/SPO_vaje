poly    START   0 .x⁴+2x³+3x²+4x+5 v točki x=2.
        LDA x
        RMO A,B     .B=x
        MUL a4      .A=a4*x

        ADD a3      .A= a3 + x*a4
        MULR B,A

        ADD a2      .A= a2 + x(a3 + x*a4)
        MULR B,A

        ADD a1      .A= a1 + (a2 + x(a3 + x*a4))
        MULR B,A

        ADD a0
        STA res
halt    J halt

x       WORD    2
a0      WORD    5
a1      WORD    4
a2      WORD    3
a3      WORD    2
a4      WORD    1
res     RESW    1

