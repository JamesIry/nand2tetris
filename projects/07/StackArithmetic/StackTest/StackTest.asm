// push constant 17
@17
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 17
@17
D=A
@SP
M=M+1
A=M-1
M=D
// eq
@SP
M=M-1
A=M
D=M
A=A-1
D=M-D
@StackTest.true.0
D;JEQ
D=0
@StackTest.join.0
0;JMP
(StackTest.true.0)
D=-1
(StackTest.join.0)
@SP
A=M-1
M=D
// push constant 17
@17
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 16
@16
D=A
@SP
M=M+1
A=M-1
M=D
// eq
@SP
M=M-1
A=M
D=M
A=A-1
D=M-D
@StackTest.true.1
D;JEQ
D=0
@StackTest.join.1
0;JMP
(StackTest.true.1)
D=-1
(StackTest.join.1)
@SP
A=M-1
M=D
// push constant 16
@16
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 17
@17
D=A
@SP
M=M+1
A=M-1
M=D
// eq
@SP
M=M-1
A=M
D=M
A=A-1
D=M-D
@StackTest.true.2
D;JEQ
D=0
@StackTest.join.2
0;JMP
(StackTest.true.2)
D=-1
(StackTest.join.2)
@SP
A=M-1
M=D
// push constant 892
@892
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 891
@891
D=A
@SP
M=M+1
A=M-1
M=D
// lt
@SP
M=M-1
A=M
D=M
A=A-1
D=M-D
@StackTest.true.3
D;JLT
D=0
@StackTest.join.3
0;JMP
(StackTest.true.3)
D=-1
(StackTest.join.3)
@SP
A=M-1
M=D
// push constant 891
@891
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 892
@892
D=A
@SP
M=M+1
A=M-1
M=D
// lt
@SP
M=M-1
A=M
D=M
A=A-1
D=M-D
@StackTest.true.4
D;JLT
D=0
@StackTest.join.4
0;JMP
(StackTest.true.4)
D=-1
(StackTest.join.4)
@SP
A=M-1
M=D
// push constant 891
@891
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 891
@891
D=A
@SP
M=M+1
A=M-1
M=D
// lt
@SP
M=M-1
A=M
D=M
A=A-1
D=M-D
@StackTest.true.5
D;JLT
D=0
@StackTest.join.5
0;JMP
(StackTest.true.5)
D=-1
(StackTest.join.5)
@SP
A=M-1
M=D
// push constant 32767
@32767
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 32766
@32766
D=A
@SP
M=M+1
A=M-1
M=D
// gt
@SP
M=M-1
A=M
D=M
A=A-1
D=M-D
@StackTest.true.6
D;JGT
D=0
@StackTest.join.6
0;JMP
(StackTest.true.6)
D=-1
(StackTest.join.6)
@SP
A=M-1
M=D
// push constant 32766
@32766
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 32767
@32767
D=A
@SP
M=M+1
A=M-1
M=D
// gt
@SP
M=M-1
A=M
D=M
A=A-1
D=M-D
@StackTest.true.7
D;JGT
D=0
@StackTest.join.7
0;JMP
(StackTest.true.7)
D=-1
(StackTest.join.7)
@SP
A=M-1
M=D
// push constant 32766
@32766
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 32766
@32766
D=A
@SP
M=M+1
A=M-1
M=D
// gt
@SP
M=M-1
A=M
D=M
A=A-1
D=M-D
@StackTest.true.8
D;JGT
D=0
@StackTest.join.8
0;JMP
(StackTest.true.8)
D=-1
(StackTest.join.8)
@SP
A=M-1
M=D
// push constant 57
@57
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 31
@31
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 53
@53
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
// push constant 112
@112
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
// neg
@SP
A=M-1
M=-M
// and
@SP
M=M-1
A=M
D=M
A=A-1
M=D&M
// push constant 82
@82
D=A
@SP
M=M+1
A=M-1
M=D
// or
@SP
M=M-1
A=M
D=M
A=A-1
M=D|M
// not
@SP
A=M-1
M=!M
