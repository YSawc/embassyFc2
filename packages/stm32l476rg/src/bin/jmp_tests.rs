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

pub fn test_jsr_abs<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xF5, 0xC5]);
    check_valid_register_status(usart, TxReg::PC, &[0xF5, 0xc5]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x20, 0x2D, 0xC7]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFF, 0x01, 0xC5]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFE, 0x01, 0xF8]);
    check_valid_register_status(usart, TxReg::PC, &[0x2D, 0xC7]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_jsr_abs passed!");
}

pub fn test_jmp_abs<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0x4c]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0xf5, 0xc5]);
    check_valid_register_status(usart, TxReg::PC, &[0xf5, 0xc5]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_jmp_abs passed!");
}

pub fn test_jmp_ind<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0x6c]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0x00, 0x02]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x00]);
    usart.blocking_write(&[0x7e]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x02, 0x00]);
    usart.blocking_write(&[0xdb]).unwrap();
    check_valid_register_status(usart, TxReg::PC, &[0x7e, 0xdb]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_jmp_ind passed!");
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
    test_jsr_abs(&mut usart, &nop, &mut resb);
    test_jmp_abs(&mut usart, &nop, &rw, &mut resb);
    test_jmp_ind(&mut usart, &nop, &rw, &mut resb);
    info!("all tests passed!");
    loop {}
}
