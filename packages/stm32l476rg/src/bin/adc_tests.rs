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

pub fn test_adc_indx<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x61, 0x80]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x80, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x81, 0x00]);
    usart.blocking_write(&[0x02]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x02]);
    usart.blocking_write(&[0x69]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x69]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_adc_indx passed!");
}

pub fn test_adc_zp<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xA9, 0x7F],
    );
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x65, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x7F]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xFE]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);
    info!("test_adc_zp passed!");
}

pub fn test_adc_imm_without_carry<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x69, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_adc_imm_without_carry passed!");
}

pub fn test_adc_imm_with_carry<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x69, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b01000011]);
    info!("test_adc_imm_with_carry passed!");
}

pub fn test_adc_imm_plus_carry<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x69, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_adc_imm_plus_carry passed!");
}

pub fn test_adc_imm_with_overflow<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x69, 0x1]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);
    info!("test_adc_imm_with_overflow passed!");
}

pub fn test_adc_abs<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x6D, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x69]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x69]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_adc_abs passed!");
}

pub fn test_adc_indy<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x71, 0x33]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x34, 0x00]);
    usart.blocking_write(&[0x04]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0x80]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    info!("test_adc_indy passed!");
}

pub fn test_adc_zpx<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x75, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x69]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x69]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_adc_zpx passed!");
}

pub fn test_adc_absy<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xFF]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x79, 0x00, 0x04]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0x80]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    info!("test_adc_absy passed!");
}

pub fn test_adc_absx<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xFF]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x7D, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x80]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    info!("test_adc_absx passed!");
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
    test_adc_indx(&mut usart, &nop, &mut resb);
    test_adc_zp(&mut usart, &nop, &mut resb);
    test_adc_imm_without_carry(&mut usart, &nop, &mut resb);
    test_adc_imm_with_carry(&mut usart, &nop, &mut resb);
    test_adc_imm_plus_carry(&mut usart, &nop, &mut resb);
    test_adc_imm_with_overflow(&mut usart, &nop, &mut resb);
    test_adc_abs(&mut usart, &nop, &mut resb);
    test_adc_indy(&mut usart, &nop, &mut resb);
    test_adc_zpx(&mut usart, &nop, &mut resb);
    test_adc_absy(&mut usart, &nop, &mut resb);
    test_adc_absx(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
