.section .text.entry
.globl _start
_start:
    la sp, boot_stack_top   # run time env setup. kinda like crt.S
    call rust_main          # jump to rust

.section .bss.stack
.globl boot_stack
boot_stack:
    .space 4096*16          # stack space we use on start

.globl boot_stack_top
boot_stack_top:             # stack top. Grows downward.