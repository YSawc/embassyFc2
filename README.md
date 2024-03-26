#### embassy fc2
famicom simulator written with rust and [veryl](https://github.com/dalance/veryl).
This repository includes embed software and hardware projects.
Currently software is used to send signals for cpu test.

#### examples

Before run below samples, there are some step needs.
1. program FPGA with veryls/6502 HDL files.
2. connect stm32l476rg and FPGA with usart.(default pin function is below.)
3. run software

##### single test runner
```
cd packages/stm32l476rg
cargo run --bin jmp_tests
```

If test passed, message shown as below. If test not passed, invalid status message shown or signal blocking will occure.
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
To check tests at once, run test-executor.
```
> cd packages/test-executor
> cargo run
   Compiling test-executor v0.1.0 (/home/ys/workspace/Rust/embassyFc2/packages/test-executor)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/test-executor`
   Compiling stm32l476rg v0.1.0 (/home/ys/workspace/Rust/embassyFc2/packages/stm32l476rg)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `probe-run --chip STM32L476rg target/thumbv7em-none-eabi/debug/cl_tests`
(HOST) INFO  flashing program (83 pages / 83.00 KiB)
(HOST) INFO  success!
(HOST) WARN  `defmt::timestamp!` implementation was found, but timestamp is not part of the log format; consider adding the timestamp `{t}` argument to the log format
────────────────────────────────────────────────────────────────────────────────
cl_tests passed.
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `probe-run --chip STM32L476rg target/thumbv7em-none-eabi/debug/cmp_tests`
(HOST) INFO  flashing program (83 pages / 83.00 KiB)
(HOST) INFO  success!
(HOST) WARN  `defmt::timestamp!` implementation was found, but timestamp is not part of the log format; consider adding the timestamp `{t}` argument to the log format
────────────────────────────────────────────────────────────────────────────────
cmp_tests passed.
...

```

#### default pin function.

|stm32|FPGA|description|
|-|-|-|
|A0|GPIO4|RW(LOW=write, HIGH=read)|
|A1|GPIO5|Nop(Low if cpumode in fpga is not nop)|
|PA4|GPIO6|Resb|
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

- In design software
  - loop limitation must be over 8192 for internal ram.
