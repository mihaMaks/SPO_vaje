prog    START   0
read    RD      #3
        COMP    #0
        JEQ     halt
        WD      #5
        JGT     read
halt    J       halt
        END     prog

