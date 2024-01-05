#### embassy fc2
famicom simulator written with rust and [veryl](https://github.com/dalance/veryl).
This repository includes embed software and hardware projects.

#### examples
- callback test
1. run veryls/6502 in simulator
2. connect stm32l476rg and DE0-CV with uart.(In default setting, stm32 pin A0 and A1 of stm32 is RX and TX, and GPIO 0-0 and 0-1 of DE0-CV is RX and TX)
3. run software
```
cd packages/stm32l476rg
cargo run --bin callback_test
```

If test passed, passed message shown as below.
```
...
INFO  wrote mode
└─ callback_test::__cortex_m_rt_main @ src/bin/callback_test.rs:31
INFO  test passed!
└─ callback_test::__cortex_m_rt_main @ src/bin/callback_test.rs:38
INFO  wait kill..
└─ callback_test::__cortex_m_rt_main @ src/bin/callback_test.rs:41
```
