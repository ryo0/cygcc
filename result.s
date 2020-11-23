.intel_syntax noprefix
  .globl main
main:
  push rbp
  mov rbp, rsp
  sub rsp, 0
  mov rax, 0
  call ret32
  jmp .L.return.main
.L.return.main:
  mov rbp, rsp
  pop rbp
  ret
  .globl ret32
ret32:
  push rbp
  mov rbp, rsp
  sub rsp, 0
  push 32
  jmp .L.return.ret32
.L.return.ret32:
  mov rbp, rsp
  pop rbp
  ret
