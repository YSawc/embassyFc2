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

pub fn test_stx_zp<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xa2, 0xaa],
    );
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x86]);
    check_rw_is_low(&rw);
    usart.blocking_write(&[0x45]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x45, 0x00, 0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_stx_zp passed!");
}

pub fn test_stx_abs<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xA2, 0xA0],
    );
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8E]);
    check_rw_is_low(&rw);
    usart_write(usart, &[0xBB, 0x03]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xBB, 0x03, 0xA0]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_stx_abs passed!");
}

pub fn test_stx_zpy<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart.blocking_write(&[CpuMode::Debug as u8]).unwrap();

    // store 0x6c to x
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0x6c]);

    // store 0xc0 to y
    usart_write(usart, &[OpeMode::Inst as u8, 0xa0, 0x34]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);

    usart_write(usart, &[OpeMode::Inst as u8, 0x96]);
    check_rw_is_low(&rw);
    usart.blocking_write(&[0x22]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x56, 0x00, 0x6C]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);

    info!("test_stx_zpy passed!");
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
    test_stx_zp(&mut usart, &nop, &rw, &mut resb);
    test_stx_abs(&mut usart, &nop, &rw, &mut resb);
    test_stx_zpy(&mut usart, &nop, &rw, &mut resb);
    info!("all tests passed!");
    loop {}
}
