// // Sys.vm for NestedCall test.
// 
// // Sys.init()
// //
// // Calls Sys.main() and stores return value in temp 1.
// // Does not return.  (Enters infinite loop.)
// 
// function Sys.init 0
(Sys.init)
// push constant 4000
@4000
D=A
@SP
M=M+1
A=M-1
M=D
// pop pointer 0
@SP
M=M-1
A=M
D=M
@THIS
M=D
// push constant 5000
@5000
D=A
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
// call Sys.main 0
@SP
D=M
@0
D=D-A
@13
M=D
@Sys.main$ret0
D=A
@SP
M=M+1
A=M-1
M=D
@LCL
D=M
@SP
M=M+1
A=M-1
M=D
@ARG
D=M
@SP
M=M+1
A=M-1
M=D
@THIS
D=M
@SP
M=M+1
A=M-1
M=D
@THAT
D=M
@SP
M=M+1
A=M-1
M=D
@R13
D=M
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.main
0;JMP
(Sys.main$ret0)
// pop temp 1
@R5
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
// label LOOP
(Sys.init$LOOP)
// goto LOOP
@Sys.init$LOOP
0;JMP
// 
// // Sys.main()
// //
// // Sets locals 1, 2 and 3, leaving locals 0 and 4 unchanged to test
// // default local initialization to 0.  (RAM set to -1 by test setup.)
// // Calls Sys.add12(123) and stores return value (135) in temp 0.
// // Returns local 0 + local 1 + local 2 + local 3 + local 4 (456) to confirm
// // that locals were not mangled by function call.
// 
// function Sys.main 5
(Sys.main)
@0
D=A
@SP
M=M+1
A=M-1
M=D
@0
D=A
@SP
M=M+1
A=M-1
M=D
@0
D=A
@SP
M=M+1
A=M-1
M=D
@0
D=A
@SP
M=M+1
A=M-1
M=D
@0
D=A
@SP
M=M+1
A=M-1
M=D
// push constant 4001
@4001
D=A
@SP
M=M+1
A=M-1
M=D
// pop pointer 0
@SP
M=M-1
A=M
D=M
@THIS
M=D
// push constant 5001
@5001
D=A
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
// push constant 200
@200
D=A
@SP
M=M+1
A=M-1
M=D
// pop local 1
@LCL
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
// push constant 40
@40
D=A
@SP
M=M+1
A=M-1
M=D
// pop local 2
@LCL
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
// push constant 6
@6
D=A
@SP
M=M+1
A=M-1
M=D
// pop local 3
@LCL
D=M
@3
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
// push constant 123
@123
D=A
@SP
M=M+1
A=M-1
M=D
// call Sys.add12 1
@SP
D=M
@1
D=D-A
@13
M=D
@Sys.add12$ret0
D=A
@SP
M=M+1
A=M-1
M=D
@LCL
D=M
@SP
M=M+1
A=M-1
M=D
@ARG
D=M
@SP
M=M+1
A=M-1
M=D
@THIS
D=M
@SP
M=M+1
A=M-1
M=D
@THAT
D=M
@SP
M=M+1
A=M-1
M=D
@R13
D=M
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.add12
0;JMP
(Sys.add12$ret0)
// pop temp 0
@SP
M=M-1
A=M
D=M
@R5
M=D
// push local 0
@LCL
A=M
D=M
@SP
M=M+1
A=M-1
M=D
// push local 1
@1
D=A
@LCL
A=M
A=A+D
D=M
@SP
M=M+1
A=M-1
M=D
// push local 2
@2
D=A
@LCL
A=M
A=A+D
D=M
@SP
M=M+1
A=M-1
M=D
// push local 3
@3
D=A
@LCL
A=M
A=A+D
D=M
@SP
M=M+1
A=M-1
M=D
// push local 4
@4
D=A
@LCL
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
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=D+M
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=D+M
// add
@SP
M=M-1
A=M
D=M
A=A-1
M=D+M
// return
@5
D=A
@LCL
D=M-D
@R13
M=D
A=D
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M+1
@SP
M=D
@R13
M=M+1
A=M
D=M
@LCL
M=D
@R13
M=M+1
A=M
D=M
@ARG
M=D
@R13
M=M+1
A=M
D=M
@THIS
M=D
@R13
M=M+1
A=M
D=M
@THAT
M=D
@R14
A=M
0;JMP
// 
// // Sys.add12(int n)
// //
// // Returns n+12.
// 
// function Sys.add12 0
(Sys.add12)
// push constant 4002
@4002
D=A
@SP
M=M+1
A=M-1
M=D
// pop pointer 0
@SP
M=M-1
A=M
D=M
@THIS
M=D
// push constant 5002
@5002
D=A
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
// push argument 0
@ARG
A=M
D=M
@SP
M=M+1
A=M-1
M=D
// push constant 12
@12
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
// return
@5
D=A
@LCL
D=M-D
@R13
M=D
A=D
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M+1
@SP
M=D
@R13
M=M+1
A=M
D=M
@LCL
M=D
@R13
M=M+1
A=M
D=M
@ARG
M=D
@R13
M=M+1
A=M
D=M
@THIS
M=D
@R13
M=M+1
A=M
D=M
@THAT
M=D
@R14
A=M
0;JMP
