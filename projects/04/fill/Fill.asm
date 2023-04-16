// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.



// fill screen loop
// r0 = next word to be filled
// r1 = pattern to fill
// r3 = temp storage

// initialize R0 to 8191, the last spot to fill
(INIT)
@8191
D=A
@R0
M=D

(POLL)
// default the fill to 0
@0
D=A
@R1
M=D
// get the keyboard
@KBD
D=M
// jump to fill if no keypress
@LOOP
D;JEQ
// if there was a keypress, set the fill to -1 and fall through
@0
D=A
D=D-1
@R1
M=D


(FILL)
// put stuff at the next spot
// load the offset
@R0
D=M
// calculate the address
@SCREEN
A=A+D
// store that address to R3
D=A
@R3
M=D
// load the filler
@R1
D=M
// write the filler
@R3
A=M
M=D
// decrement R0
@R0
M=M-1
D=M

//loop back around if we've got more to go
@POLL
D;JGE

//otherwise, reset
@INIT
0;JMP