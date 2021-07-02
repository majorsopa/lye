extern _GetStdHandle@4
extern _WriteFile@20
extern malloc

extern _ExitProcess@4

global Start

section .text

Start:
    mov edx, testyy_0123456789
    push edx
    mov ecx,0
    dec edx
    count:
        inc ecx
        inc edx
        cmp byte[edx], 0
        jnz count
    dec ecx
    pop edx

    push    -11
    call    _GetStdHandle@4
    mov     ebx, eax

    push    0
    lea     eax, [ebp-4]
    push    eax
    push    ecx
    push    testyy_0123456789
    push    ebx
    call    _WriteFile@20

    jmp end_program

end_program:
    push 0
    call _ExitProcess@4

section .data
    testyy_0123456789 db 'Hello World!'
