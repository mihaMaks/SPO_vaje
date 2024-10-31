CAT     START 0
WAIT    TD	    #0
	    JEQ	    WAIT
        
        RD      #0           . Preberemo znak iz vhoda
        COMP    #0           . Preverimo, ali je konec datoteke
        JEQ     END          . Če je konec, končamo program
        WD #1                   . Drugače izpišemo znak na izhod
        
        J       WAIT         . Gremo nazaj na branje
END     END CAT