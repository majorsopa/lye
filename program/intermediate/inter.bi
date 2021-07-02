extern _GetStdHandle@4
extern _WriteFile@20

extern _ExitProcess@4

global Start

section .text

Start:
    jmp end_program

end_program:
    push 0
    call _ExitProcess@4

section .data
    testyy0_0123456789 db 'Hello World!...', 0
