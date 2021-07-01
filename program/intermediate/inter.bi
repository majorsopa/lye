extern _GetStdHandle@4
extern _WriteFile@20
extern _ExitProcess@4

global Start

section .text

Start:

end_program:
    push 0
    call _ExitProcess@4

section .data
    test0 db "Hello World!"
    test2 equ 42
    test3 equ 1
