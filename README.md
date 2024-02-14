#### embassy fc2
famicom simulator written with rust and [veryl](https://github.com/dalance/veryl).
This repository includes embed software and hardware projects.
Currently software is used to send signals for cpu test.

#### examples

Before run below samples, there are some step needs.
1. run veryls/6502 in simulator
2. connect stm32l476rg and DE0-CV with usart.(default pin function is below.)
3. run software

##### single test runner
```
cd packages/stm32l476rg
cargo run --bin jmp_abs_test
```

If test passed, passed message shown as below. If test not passed, fpga requires reset for initializing internal state machine and registers.
```
...
INFO  rw flag is high
└─ jmp_abs_test::__cortex_m_rt_main @ src/bin/jmp_abs_test.rs:36
INFO  write target memory row.
└─ jmp_abs_test::__cortex_m_rt_main @ src/bin/jmp_abs_test.rs:44
INFO  write target memory high.
└─ jmp_abs_test::__cortex_m_rt_main @ src/bin/jmp_abs_test.rs:47
INFO  write operation mode.
└─ jmp_abs_test::__cortex_m_rt_main @ src/bin/jmp_abs_test.rs:50
INFO  write tx reg.
└─ jmp_abs_test::__cortex_m_rt_main @ src/bin/jmp_abs_test.rs:53
INFO  test passed!
└─ jmp_abs_test::__cortex_m_rt_main @ src/bin/jmp_abs_test.rs:5
```

##### multiple test runner
Package prepared to run many test once.
```
cd packages/test-executor
cargo run
...
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s                           Running `target/debug/test-executor`
jmp_abs_test passed.
jmp_ind_test passed.
lda_zp_test passed
...

```

#### default pin function.

|stm32|de0-cv|description|
|-|-|-|
|A0|GPIO4|RW(LOW=write, HIGH=read)|
|A1|GPIO5|Nop(Low if cpumode in fpga is not nop)|
|PA2|GPIO0|RX|
|PA9|GPIO1|TX|
|PA12|GPIO2|CTS|
|PA11|GPIO3|RTS|

#### requirements
- usbblaster rules
```
cat /etc/udev/rules.d/51-usbblaster.rules
# USB-Blaster
SUBSYSTEM=="usb", ATTRS{idVendor}=="09fb", ATTRS{idProduct}=="6001", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="09fb", ATTRS{idProduct}=="6002", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="09fb", ATTRS{idProduct}=="6003", MODE="0666"

# USB-Blaster II
SUBSYSTEM=="usb", ATTRS{idVendor}=="09fb", ATTRS{idProduct}=="6010", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="09fb", ATTRS{idProduct}=="6810", MODE="0666"
```

- stm32 rules
```
❯ cat /etc/udev/rules.d/49-stm32.rules
# 0483:5740 - STM32F4 Dsicovery in USB Serial Mode (CN5)
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="5740", ENV{ID_MM_DEVICE_IGNORE}="1"
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="5740", ENV{MTP_NO_PROBE}="1"
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="5740", MODE:="0666"
KERNEL=="ttyACM*", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="5740", MODE:="0666"

# 0483:df11 - STM32F4 Discovery in DFU mode (CN5)
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="df11", MODE:="0666"
```
