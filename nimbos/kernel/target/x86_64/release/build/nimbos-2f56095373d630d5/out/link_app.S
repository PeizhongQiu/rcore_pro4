
.section .data
.balign 8
.global _app_count
_app_count:
    .quad 20
    .quad app_0_name
    .quad app_0_start
    .quad app_1_name
    .quad app_1_start
    .quad app_2_name
    .quad app_2_start
    .quad app_3_name
    .quad app_3_start
    .quad app_4_name
    .quad app_4_start
    .quad app_5_name
    .quad app_5_start
    .quad app_6_name
    .quad app_6_start
    .quad app_7_name
    .quad app_7_start
    .quad app_8_name
    .quad app_8_start
    .quad app_9_name
    .quad app_9_start
    .quad app_10_name
    .quad app_10_start
    .quad app_11_name
    .quad app_11_start
    .quad app_12_name
    .quad app_12_start
    .quad app_13_name
    .quad app_13_start
    .quad app_14_name
    .quad app_14_start
    .quad app_15_name
    .quad app_15_start
    .quad app_16_name
    .quad app_16_start
    .quad app_17_name
    .quad app_17_start
    .quad app_18_name
    .quad app_18_start
    .quad app_19_name
    .quad app_19_start
    .quad app_19_end

app_0_name:
    .string "cyclictest"
.balign 8
app_0_start:
    .incbin "../user/build/x86_64/cyclictest"
app_0_end:

app_1_name:
    .string "exit"
.balign 8
app_1_start:
    .incbin "../user/build/x86_64/exit"
app_1_end:

app_2_name:
    .string "fantastic_text"
.balign 8
app_2_start:
    .incbin "../user/build/x86_64/fantastic_text"
app_2_end:

app_3_name:
    .string "forktest"
.balign 8
app_3_start:
    .incbin "../user/build/x86_64/forktest"
app_3_end:

app_4_name:
    .string "forktest2"
.balign 8
app_4_start:
    .incbin "../user/build/x86_64/forktest2"
app_4_end:

app_5_name:
    .string "forktest_simple"
.balign 8
app_5_start:
    .incbin "../user/build/x86_64/forktest_simple"
app_5_end:

app_6_name:
    .string "forktest_simple_c"
.balign 8
app_6_start:
    .incbin "../user/build/x86_64/forktest_simple_c"
app_6_end:

app_7_name:
    .string "forktree"
.balign 8
app_7_start:
    .incbin "../user/build/x86_64/forktree"
app_7_end:

app_8_name:
    .string "get_char"
.balign 8
app_8_start:
    .incbin "../user/build/x86_64/get_char"
app_8_end:

app_9_name:
    .string "hello_c"
.balign 8
app_9_start:
    .incbin "../user/build/x86_64/hello_c"
app_9_end:

app_10_name:
    .string "hello_world"
.balign 8
app_10_start:
    .incbin "../user/build/x86_64/hello_world"
app_10_end:

app_11_name:
    .string "matrix"
.balign 8
app_11_start:
    .incbin "../user/build/x86_64/matrix"
app_11_end:

app_12_name:
    .string "put_char"
.balign 8
app_12_start:
    .incbin "../user/build/x86_64/put_char"
app_12_end:

app_13_name:
    .string "sleep"
.balign 8
app_13_start:
    .incbin "../user/build/x86_64/sleep"
app_13_end:

app_14_name:
    .string "sleep_simple"
.balign 8
app_14_start:
    .incbin "../user/build/x86_64/sleep_simple"
app_14_end:

app_15_name:
    .string "stack_overflow"
.balign 8
app_15_start:
    .incbin "../user/build/x86_64/stack_overflow"
app_15_end:

app_16_name:
    .string "thread_simple"
.balign 8
app_16_start:
    .incbin "../user/build/x86_64/thread_simple"
app_16_end:

app_17_name:
    .string "user_shell"
.balign 8
app_17_start:
    .incbin "../user/build/x86_64/user_shell"
app_17_end:

app_18_name:
    .string "usertests"
.balign 8
app_18_start:
    .incbin "../user/build/x86_64/usertests"
app_18_end:

app_19_name:
    .string "yield"
.balign 8
app_19_start:
    .incbin "../user/build/x86_64/yield"
app_19_end:
