#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode, TxReg};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Pull};
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
    send_reset_signal_if_not_nop(&mut usart, &nop);
    usart_write(
        &mut usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xa9, 0xbb],
    );
    check_valid_register_status(&mut usart, TxReg::P, &[0b10000000]);
    usart_write(&mut usart, &[OpeMode::Inst as u8, 0x85]);
    check_rw_is_low(rw);
    usart.blocking_write(&[0x45]).unwrap();
    usart_read_with_check(&mut usart, &mut [0x0u8; 2], &[0x45, 0xBB]);
    check_valid_register_status(&mut usart, TxReg::P, &[0b10000000]);
    info!("test passed!");
    loop {}
}
