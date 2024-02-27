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

pub fn test_dec_zp<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC6, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_dec_zp passed!");
}

pub fn test_dec_abs<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCE, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x80]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_dec_abs passed!");
}

pub fn test_dec_zpx<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xDE, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x06]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_dec_zpx passed!");
}

pub fn test_dec_absx<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xDE, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x06]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_dec_absx passed!");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let config = Config::default();
    let mut usart = Uart::new_with_rtscts(
        p.USART1, p.PA10, p.PA9, Irqs, p.PA12, p.PA11, NoDma, NoDma, config,
    )
    .unwrap();
    let _rw = Input::new(p.PA0, Pull::None);
    let nop = Input::new(p.PA1, Pull::None);
    let mut resb = Output::new(p.PA4, Level::Low, Speed::Medium);
    test_dec_zp(&mut usart, &nop, &mut resb);
    test_dec_abs(&mut usart, &nop, &mut resb);
    test_dec_zpx(&mut usart, &nop, &mut resb);
    test_dec_absx(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
