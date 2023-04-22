// // This file is part of www.nand2tetris.org
// // and the book "The Elements of Computing Systems"
// // by Nisan and Schocken, MIT Press.
// // File name: projects/08/ProgramFlow/FibonacciSeries/FibonacciSeries.vm
// 
// // Puts the first argument[0] elements of the Fibonacci series
// // in the memory, starting in the address given in argument[1].
// // Argument[0] and argument[1] are initialized by the test script 
// // before this code starts running.
// 
// push argument 1
@1
D=A
@ARG
A=M
A=A+D
D=M
@SP
M=M+1
A=M-1
M=D
// pop pointer 1
@THIS
D=A
@1
D=D+A
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D
// 
// push constant 0
@0
D=A
@SP
M=M+1
A=M-1
M=D
// pop that 0
@SP
M=M-1
A=M
D=M
@THAT
A=M
M=D
// push constant 1
@1
D=A
@SP
M=M+1
A=M-1
M=D
// pop that 1
@THAT
D=M
@1
D=D+A
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D
// 
// push argument 0
@ARG
A=M
D=M
@SP
M=M+1
A=M-1
M=D
// push constant 2
@2
D=A
@SP
M=M+1
A=M-1
M=D
// sub
@SP
M=M-1
A=M
D=M
A=A-1
M=M-D
// pop argument 0
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
// 
// label MAIN_LOOP_START
(FibonacciSeries$MAIN_LOOP_START)
// 
// push argument 0
@ARG
A=M
D=M
@SP
M=M+1
A=M-1
M=D
// if-goto COMPUTE_ELEMENT
@SP
M=M-1
A=M
D=M
@FibonacciSeries$COMPUTE_ELEMENT
D;JNE
// goto END_PROGRAM
@FibonacciSeries$END_PROGRAM
0;JMP
// 
// label COMPUTE_ELEMENT
(FibonacciSeries$COMPUTE_ELEMENT)
// 
// push that 0
@THAT
A=M
D=M
@SP
M=M+1
A=M-1
M=D
// push that 1
@1
D=A
@THAT
A=M
A=A+D
D=M
@SP
M=M+1
A=M-1
M=D
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=D+M
// pop that 2
@THAT
D=M
@2
D=D+A
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D
// 
// push pointer 1
@1
D=A
@THIS
A=A+D
D=M
@SP
M=M+1
A=M-1
M=D
// push constant 1
@1
D=A
@SP
M=M+1
A=M-1
M=D
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=D+M
// pop pointer 1
@THIS
D=A
@1
D=D+A
@R13
M=D
@SP
M=M-1
A=M
D=M
@R13
A=M
M=D
// 
// push argument 0
@ARG
A=M
D=M
@SP
M=M+1
A=M-1
M=D
// push constant 1
@1
D=A
@SP
M=M+1
A=M-1
M=D
// sub
@SP
M=M-1
A=M
D=M
A=A-1
M=M-D
// pop argument 0
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
// 
// goto MAIN_LOOP_START
@FibonacciSeries$MAIN_LOOP_START
0;JMP
// 
// label END_PROGRAM
(FibonacciSeries$END_PROGRAM)
