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

pub fn test_lda_indx<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x18]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x18]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9]);
    usart.blocking_write(&[0x40]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA1]);
    usart.blocking_write(&[0x1B]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0xFF]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x34, 0x00]);
    usart.blocking_write(&[0xF0]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFF, 0xF0]);
    usart.blocking_write(&[0xCF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xCF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_lda_indx passed!");
}

pub fn test_lda_zp<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xa5]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x25]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x25, 0x00]);
    usart.blocking_write(&[0x45]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x45]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_lda_zp passed!");
}

pub fn test_lda_imm<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xa9]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x34]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x34]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_lda_imm passed!");
}

pub fn test_lda_indy<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xA0, 0x00],
    );
    check_valid_register_status(usart, TxReg::Y, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB1]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0x89]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x89, 0x00]);
    usart_write(usart, &[0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x8A, 0x00]);
    usart_write(usart, &[0x03]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x03]);
    usart_write(usart, &[0x89]);
    check_valid_register_status(usart, TxReg::A, &[0x89]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_lda_indy passed!");
}

pub fn test_lda_zpx<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x05]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x05]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB5, 0x60]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x65, 0x00]);
    usart.blocking_write(&[0x0C]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x0C]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_lda_zpx passed!");
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
    send_reset_signal_if_not_nop(&nop, &mut resb);
    usart_write(
        &mut usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xa9],
    );
    test_lda_indx(&mut usart, &nop, &rw, &mut resb);
    test_lda_zp(&mut usart, &nop, &rw, &mut resb);
    test_lda_imm(&mut usart, &nop, &rw, &mut resb);
    test_lda_indy(&mut usart, &nop, &rw, &mut resb);
    test_lda_zpx(&mut usart, &nop, &rw, &mut resb);
    info!("all tests passed!");
    loop {}
}
