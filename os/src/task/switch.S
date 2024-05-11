.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch
__switch:
    # __switch( Task A => Task B
    #   current_task_cx_ptr: *mut TaskContext(a0) Task A
    #   next_task_cx_ptr: *const TaskContext(a1) Task B 
    #)
    sd sp, 8(a0) # save kernel stack for A
    sd ra, 0(a0) # save A's ra
    .set n, 0    # save A's saved registers
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr
    ld ra, 0(a1) # restore B's ra
    .set n, 0    # restore B's saved registers
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    ld sp, 8(a1) # restore B's kernel stack
    ret