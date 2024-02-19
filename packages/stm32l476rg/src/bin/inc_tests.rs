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

pub fn test_inc_abs<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xee, 0x00, 0x04],
    );
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0x40]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x41]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_inc_abs passed!");
}

pub fn test_inc_absx_without_carry<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart.blocking_write(&[CpuMode::Debug as u8]).unwrap();

    // store 0x30 to x
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0x30]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xFE, 0x30, 0x40]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x60, 0x40]);
    usart_write(usart, &[0x60]);
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x61]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_inc_abs_x_without_carry passed!");
}

pub fn test_inc_absx_with_carry<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart.blocking_write(&[CpuMode::Debug as u8]).unwrap();

    // store 0xff to x
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0xff]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xFE, 0x30, 0x40]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2F, 0x41]);
    usart_write(usart, &[0x60]);
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x61]);
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
    info!("test_inc_abs_x_with_carry passed!");
}

pub fn test_inc_zp_without_triger_of_p<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xe6, 0x2c],
    );
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2C, 0x00]);
    usart.blocking_write(&[0x7e]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_inc_zp_without_triger_of_p passed!");
}

pub fn test_inc_zp_with_overflow_and_zero_flag<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xe6, 0x2c],
    );
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2C, 0x00]);
    usart.blocking_write(&[0xff]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b01000010]);
    info!("test_inc_zp_with_zero_flag passed!");
}

pub fn test_inc_zp_with_negative_flag<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xe6, 0x2c],
    );
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2C, 0x00]);
    usart.blocking_write(&[0x7f]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_inc_zp_with_overflow_and_zero_flag passed!");
}

pub fn test_inc_zpx<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart.blocking_write(&[CpuMode::Debug as u8]).unwrap();
    info!("write cpu operation mode.");

    // store to x
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0x50]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xf6, 0x67]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xB7, 0x00]);
    usart.blocking_write(&[0xa0]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0xA1]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);

    info!("test_inc_zpx passed!");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let config = Config::default();
    let mut usart = Uart::new_with_rtscts(
        p.USART1, p.PA10, p.PA9, Irqs, p.PA12, p.PA11, NoDma, NoDma, config,
    )
    .unwrap();
    let nop = Input::new(p.PA1, Pull::None);
    let mut resb = Output::new(p.PA4, Level::Low, Speed::Medium);
    test_inc_abs(&mut usart, &nop, &mut resb);
    test_inc_absx_without_carry(&mut usart, &nop, &mut resb);
    test_inc_absx_with_carry(&mut usart, &nop, &mut resb);
    test_inc_zp_without_triger_of_p(&mut usart, &nop, &mut resb);
    test_inc_zp_with_overflow_and_zero_flag(&mut usart, &nop, &mut resb);
    test_inc_zp_with_negative_flag(&mut usart, &nop, &mut resb);
    test_inc_zpx(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
