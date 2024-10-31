PROG    START 0
        LDA #4
        JSUB SUMN
        . V A NAJ BO VSOTA 1+2+3+4
HALT    J HALT
        END PROG

. PUSHL 


. REKURZIVNO SESTEJE 1+ ... +N (N JE PODAN V A)
SUMN    COMP #0
        PUSH L          .NAMESTO "STL SUML"
        JEQ SUMOUT
        RMO A,B         .SHRANIMO A NEKAM
        SUB #1          .ZMANJSAMO A
        JSUB SUMN       .POVOZIMO VREDNOST V L IN SE ZACIKLAMO
        ADDR B,A
SUMOUT  POP L           .NAMESTO "LDL SUML"
        RSUB
SUML    WORD 0