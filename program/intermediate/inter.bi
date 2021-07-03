extern _GetStdHandle@4
extern _WriteFile@20

extern _ExitProcess@4

global Start

section .text

Start:
    push testyy0_0123456789
    call std_print_function
    pop ecx

    push testyy1_0123456789
    call std_print_function
    pop ecx

    jmp    end_program

std_print_function:
    push    ebp
    mov     ebp, esp
    and     esp, 0xfffffff0
    mov     edx, [ebp+8]

    push    -11
    call    _GetStdHandle@4
    mov     ebx, eax

    push    0
    lea     eax, [ebp-4]
    push    eax

    call    std_string_length_getter
    push    ecx

    push    edx
    push    ebx
    call    _WriteFile@20


    mov     esp, ebp
    pop     ebp
    ret

std_string_length_getter:
    push    edx

    xor     ecx, ecx
    dec     edx
    count0:
        inc     ecx
        inc     edx
        cmp     byte[edx],0
        jnz     count0
    dec     ecx

    pop     edx
    ret



end_program:
    push    0
    call    _ExitProcess@4

section .data
    testyy0_0123456789 db `Hello World!\n`, 0
    testyy1_0123456789 db `Goodbye World!`, 0
