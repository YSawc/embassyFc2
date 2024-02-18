#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode, TxReg};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use stm32l476rg::pin::util::*;
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
    let mut resb = Output::new(p.PA4, Level::Low, Speed::Medium);
    send_reset_signal_if_not_nop(&nop, &mut resb);
    usart_write(
        &mut usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xa0],
    );
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xba]).unwrap();
    info!("write callback value.");
    check_valid_register_status(&mut usart, TxReg::Y, &[0xba]);
    check_valid_register_status(&mut usart, TxReg::P, &[0b10000000]);
    info!("test passed!");
    loop {}
}
