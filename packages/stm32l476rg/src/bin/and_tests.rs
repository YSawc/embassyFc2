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

pub fn test_and_indx<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x21, 0x82]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x82, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x83, 0x00]);
    usart.blocking_write(&[0x03]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x03]);
    usart.blocking_write(&[0xEF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_and_indx passed!");
}

pub fn test_and_zp<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x25, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    info!("test_and_zp passed!");
}

pub fn test_and_imm_without_no_flag<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x6F]);
    check_valid_register_status(usart, TxReg::A, &[0x6F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x29, 0xEF]);
    check_valid_register_status(usart, TxReg::A, &[0x6F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_and_imm_without_no_flag passed!");
}

pub fn test_and_imm_with_z<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x29, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    info!("test_and_imm_with_z passed!");
}

pub fn test_and_imm_with_n<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x67]);
    check_valid_register_status(usart, TxReg::A, &[0x67]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x29, 0x98]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    info!("test_and_imm_with_n passed!");
}

pub fn test_and_abs<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x2D, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0xEF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_and_abs passed!");
}

pub fn test_and_indy<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x31, 0x33]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x34, 0x00]);
    usart.blocking_write(&[0x04]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    info!("test_and_indy passed!");
}

pub fn test_and_zpx<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x35, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0xEF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_and_zpx passed!");
}

pub fn test_and_absy<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x39, 0x00, 0x04]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    info!("test_and_absy passed!");
}

pub fn test_and_absx<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x3D, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0xEF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_and_absx passed!");
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
    test_and_indx(&mut usart, &nop, &mut resb);
    test_and_zp(&mut usart, &nop, &mut resb);
    test_and_imm_without_no_flag(&mut usart, &nop, &mut resb);
    test_and_imm_with_z(&mut usart, &nop, &mut resb);
    test_and_imm_with_n(&mut usart, &nop, &mut resb);
    test_and_abs(&mut usart, &nop, &mut resb);
    test_and_indy(&mut usart, &nop, &mut resb);
    test_and_zpx(&mut usart, &nop, &mut resb);
    test_and_absy(&mut usart, &nop, &mut resb);
    test_and_absx(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
