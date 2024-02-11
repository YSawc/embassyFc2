#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode, TxReg};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use stm32l476rg::pin::util::{send_reset_signal_if_not_nop, check_valid_register_status};
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
    let nop = Input::new(p.PA1, Pull::None);
    let mut buf = [0x0u8; 1];
    send_reset_signal_if_not_nop(&mut usart, nop);
    buf[0] = CpuMode::Debug as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write cpu operation mode.");
    // store 0xc0 to y
    buf[0] = OpeMode::Inst as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write operation mode.");
    buf[0] = 0xa0;
    usart.blocking_write(&buf).unwrap();
    info!("write instruction.");
    buf[0] = 0xc0;
    usart.blocking_write(&buf).unwrap();
    info!("write store value to a.");

    buf[0] = OpeMode::Inst as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write operation mode.");
    buf[0] = 0xb6;
    usart.blocking_write(&buf).unwrap();
    info!("write instruction.");
    match rw.is_high() {
        true => info!("rw flag is high"),
        false => {
            info!("test failed. rw flag is not high.");
            loop {}
        }
    }
    buf[0] = 0x91;
    usart.blocking_write(&buf).unwrap();
    info!("write target address.");
    check_valid_register_status(&mut usart, TxReg::X, &[0x52]);
    check_valid_register_status(&mut usart, TxReg::P, &[0b00000000]);
    info!("test passed!");
    loop {}
}
