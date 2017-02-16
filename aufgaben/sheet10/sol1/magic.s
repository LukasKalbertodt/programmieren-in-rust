; This is the assembly of:
;
;     fn is_prime(n: u64) -> bool {
;         (2..n).all(|d| n % d != 0)
;     }
;
; Of course, it was compiled in release mode. Thus, this is a primality test
; which works for numbers >= 2.

magic:
    push    rbp         ; standard function prologue:
    mov     rbp, rsp    ;     storing old rbp and setting new one

    mov     rcx, rdi    ; copy function argument to rcx
    mov     esi, 2      ; rsi = 2 (this will be our counter)

.LBB0_1:
    mov     al, 1       ; out = true (rax contains the return value)

    cmp     rsi, rcx    ; if our counter is equal to our function argument
    jae     .LBB0_4     ; (rsi == rcx) we will jump to the end (return)

    xor     edx, edx    ; set rdx to 0 (just for the upcoming div instruction)
    mov     rax, rcx    ; copy the function argument into rax temporarily
                        ; (this is just for the next instruction)

    div     rsi         ; divide rdx:rax by rsi (counter). The result is stored
                        ; in rax and more importantly: the remainder is stored
                        ; in rdx!

    inc     rsi         ; increment our counter

    test    rdx, rdx    ; if the remainder is NOT 0, we will jump up
    jne     .LBB0_1     ; again (keep going). Else we will just continue...

    xor     eax, eax    ; out = false
.LBB0_4:
    pop     rbp         ; standard function epilogue
    ret
