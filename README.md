# Sistemska programska oprema (System software)
In the System Software course, we covered the SIC (Simple Instruction Computer) simulator and developed our own simulator for executing object code, as well as an assembler that translates .asm files into object code.
We used simulator you can find here: https://github.com/jurem/SicTools

## Assembly programs
In folder dn1 there are programs written in assembly for SIC. I would like to highlith programs ball.asm and graphical.asm.
To run this programs in simulator:
```bash
cd dn1
```
### ball.asm:
Run the simulator the ball.asm will be loaded to memory automaticaly.
```bash
java -jar sictools.jar ball.asm
```
Click `Addons` and `Toggle textual screen` then `Start` button in simulator. You should see the ball bouncing in textual screen.

You can expiriment by changind ball spped and direction by changing lines 37 and 38.

### graphical.asm
```bash
java -jar sictools.jar graphical.asm
```
Click `Addons` and `Toggle graphical screen` then `Start` button in simulator. You should see program writte on graphical screen.

You can expiriment by changind size of letters  by changing line 642.

## Simulator in rust
In `sic_xe` there is a simple simulator witten in rust that loads .obj file in memory and executes the programm. To run programs in rust simulator:
```bash
cd sic_xe
```

### ftf.obj
Program reads from file `3` and writes content to file `5`.
```bash
cargo run auto false false 1 0 ../dn1/obj/ftf.obj
```

### ball.obj 
```bash
cargo run step false false 1 1 ../dn1/obj/ball.obj
```
You can run other programs like this.
This are posible options for arguments:
`cargo run executon=<step/auto> debug=<true/false> textual_screen=<true/false> Hz=<1-100> gui=<0/1> <file_path>`
#### execution
 - step: program executes one instruction at a time
 - auto: executon is automated
#### debug
- false: only instruction name written to terminal
- true: get more information about instgruction
#### textual_screen
- false: textual screen not written to terminal
- true: after every instruction textuial scrren is writen to terminal
#### Hz 
- frequency of intruction executon
#### gui
- 0: do not toggle gui
- 1: toggle gui, you can change frequency and execution options there also a textual screen is badly implemented that i used for ball.obj

You can try running other programms but note that this was also a project where I was learning rust hence the impractical running method.

## Assebler in java
I wrote assembler in java for compiling assembly for SIC. Some code like lexer and a part of parser was already written by Jure Mihelič. 
Project root is `SPO-Asm/src/sic`.

The program reads the .asm code and compiles it to machine code and stores it to my_<file_name>.obj file.