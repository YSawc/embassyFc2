#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode, TxReg};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let config = Config::default();
    let mut usart = Uart::new_with_rtscts(
        p.USART1, p.PA10, p.PA9, Irqs, p.PA12, p.PA11, NoDma, NoDma, config,
    )
    .unwrap();
    let rw = Input::new(p.PA0, Pull::None);
    let mut buf = [0x0u8; 1];
    buf[0] = CpuMode::Debug as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write cpu operation mode.");

    // store 0x50 to x
    buf[0] = OpeMode::Inst as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write operation mode.");
    buf[0] = 0xa2;
    usart.blocking_write(&buf).unwrap();
    info!("write instruction.");
    buf[0] = 0x50;
    usart.blocking_write(&buf).unwrap();
    info!("write store value to a.");

    buf[0] = OpeMode::Inst as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write operation mode.");
    buf[0] = 0xb5;
    usart.blocking_write(&buf).unwrap();
    info!("write instruction.");
    match rw.is_high() {
        true => info!("rw flag is high"),
        false => {
            info!("test failed. rw flag is not high.");
            loop {}
        }
    }
    buf[0] = 0xb5;
    usart.blocking_write(&buf).unwrap();
    info!("write target address.");
    buf[0] = OpeMode::RegisterTransfer as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write operation mode.");
    buf[0] = TxReg::A as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write tx reg.");
    let mut read_buf = [0x0u8; 1];
    usart.blocking_read(&mut read_buf).unwrap();
    match read_buf {
        [0x06] => info!("test passed!"),
        v => {
            info!("test failed. return value is {:?}", v);
            loop {}
        }
    }
    loop {}
}
