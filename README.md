#### embassy fc2
famicom simulator written with rust and [veryl](https://github.com/dalance/veryl).
This repository includes embed software and hardware projects.

#### examples
- callback test
1. run veryls/6502 in simulator
2. connect stm32l476rg and DE0-CV with usart.(In default, stm32 pin A0 and A1 of stm32 is RX and TX, and GPIO 0-0 and 0-1 of DE0-CV is RX and TX)
3. run software
```
cd packages/stm32l476rg
cargo run --bin jmp_abs_test
```

If test passed, passed message shown as below.
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

#### default pin function.

|stm32|de0-cv|description|
|-|-|-|
|A0|GPIO4|RW|
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
