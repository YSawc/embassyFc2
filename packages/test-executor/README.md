#### summary
test runner for execute test streamly.

if fpga is ready after reset, simply run `cargo run` with outputs below.
This is utility tool for execute test for 6502 operations through usart signal of stm32.

```sh
> cargo run
   Compiling test-executor v0.1.0 (/home/ys/workspace/Rust/embassyFc2/packages/test-executor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/test-executor`
jmp_abs_test passed.
jmp_ind_test passed.
lda_zp_test passed.
lda_zpx_test passed.
ldx_imm_test passed.
ldx_zp_test passed.
ldx_imm_test passed.
ldx_zp_test passed.
ldx_zpy_test passed.
ldy_imm_test passed.
ldy_zp_test passed.
sta_zp_test passed.
sta_zpx_test passed.
stx_zp_test passed.
stx_zpy_test passed.
sty_zp_test passed.
```

If you want to run only one 6502 operation test, try to move directory of `embassyFc2/packages/stm32l476rg` and run `cargo run --bin [testcase]`.
