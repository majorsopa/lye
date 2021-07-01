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
    test db "hello world"