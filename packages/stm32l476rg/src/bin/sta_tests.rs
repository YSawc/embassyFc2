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

pub fn test_sta_indx<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xC4]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0xC4]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9]);
    usart.blocking_write(&[0x03]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x03]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x81]);
    check_rw_is_low(rw);
    usart.blocking_write(&[0x45]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x09, 0x00]);
    usart.blocking_write(&[0x59]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x0A, 0x00]);
    usart.blocking_write(&[0xBA]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x59, 0xBA, 0x03]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_sta_indx passed!");
}

pub fn test_sta_zp<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xa9, 0xbb],
    );
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x85]);
    check_rw_is_low(rw);
    usart.blocking_write(&[0x45]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x45, 0x00, 0xBB]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_sta_zp passed!");
}

pub fn test_sta_zpx<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart.blocking_write(&[CpuMode::Debug as u8]).unwrap();

    // store 0xbb to a
    usart_write(usart, &[OpeMode::Inst as u8, 0xa9, 0xbb]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);

    // store 0x11 to x
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0x11]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x95]);
    check_rw_is_low(&rw);
    usart.blocking_write(&[0x45]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x56, 0x00, 0xBB]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);

    info!("test_sta_zpx passed!");
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
    test_sta_indx(&mut usart, &nop, &rw, &mut resb);
    test_sta_zp(&mut usart, &nop, &rw, &mut resb);
    test_sta_zpx(&mut usart, &nop, &rw, &mut resb);
    info!("all tests passed!");
    loop {}
}
