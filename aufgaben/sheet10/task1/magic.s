magic:
    push    rbp
    mov     rbp, rsp
    mov     rcx, rdi
    mov     esi, 2
.LBB0_1:
    mov     al, 1
    cmp     rsi, rcx
    jae     .LBB0_4
    xor     edx, edx
    mov     rax, rcx
    div     rsi
    inc     rsi
    test    rdx, rdx
    jne     .LBB0_1
    xor     eax, eax
.LBB0_4:
    pop     rbp
    ret
