.altmacro
# save gp register to stack
.macro SAVE_GP n
    sd  x\n, \n*8(sp)
.endm
# load gp from stack
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp 
    # now sp->kernel stack, sscratch->user stack
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8 
    # save general-purpose registers
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    csrr t2, sscratch
    sd t2, 2*8(sp)
    mv a0, sp # a0<-sp (kernel)
    call trap_handler

__restore:
    # mv sp, a0 # sp<-a0 (kernel)
    # now sp->kernel stack, sscratch->user stack
    # restore user stack
    ld t0, 32*8(sp) # sstatus
    ld t1, 33*8(sp) # sepc
    ld t2, 2*8(sp) # user stack sp
    csrw sstatus, t0
    csrw sepc, t1 
    csrw sscratch, t2
    # restore gp registers
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # release TrapContext
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp
    // now sp->user stack
    sret