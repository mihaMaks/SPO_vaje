prog    START 0
        LDA 42
        LDA @42
        LDA #42
        LDA x
        LDA @x
        LDA #x
        LDA #4095
        LDB #-1 .ista vrednost v registru
        ORG 21 .naredi lukno med ukazi velikosti (2?)
        LDA =42 .nam ustvari label z vrednostjo 42
halt    J halt

x       WORD 42
        END prog
