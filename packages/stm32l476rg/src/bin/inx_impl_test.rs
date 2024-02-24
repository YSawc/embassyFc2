#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode, TxReg};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
use embassy_stm32::usart::{BasicInstance, Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use stm32l476rg::pin::util::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub fn test_inx_impl_without_flag<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x59]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x59]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xE8]);
    check_valid_register_status(usart, TxReg::X, &[0x5A]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
}

pub fn test_inx_impl_with_zero<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xFF]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xE8]);
    check_valid_register_status(usart, TxReg::X, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b01000010]);
}

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
    test_inx_impl_without_flag(&mut usart, &rw, &nop, &mut resb);
    test_inx_impl_with_zero(&mut usart, &rw, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
