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
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA2,
        ],
    );
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
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xa9,
            0xbb,
        ],
    );
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x85]);
    check_rw_is_low(rw);
    usart.blocking_write(&[0x45]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x45, 0x00, 0xBB]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_sta_zp passed!");
}

pub fn test_sta_abs<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA9,
            0x2A,
        ],
    );
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D]);
    check_rw_is_low(&rw);
    usart_write(usart, &[0x11, 0x33]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x11, 0x33, 0x2A]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_sta_abs passed!");
}

pub fn test_sta_indy<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA9,
            0xFF,
        ],
    );
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x34]);
    check_valid_register_status(usart, TxReg::Y, &[0x34]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x91]);
    check_rw_is_low(&rw);
    usart_write(usart, &[0x97]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x97, 0x00]);
    usart_write(usart, &[0xFF]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x98, 0x00]);
    usart_write(usart, &[0xFF]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x33, 0x00, 0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_sta_indy passed!");
}

pub fn test_sta_zpx<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart
        .blocking_write(&[CpuMode::DebugWithinMockMemory as u8])
        .unwrap();

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

pub fn test_sta_absy<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA0,
            0xF0,
        ],
    );
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x2A]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x99]);
    check_rw_is_low(&rw);
    usart_write(usart, &[0x23, 0x34]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x13, 0x35, 0x2A]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_sta_absy passed!");
}

pub fn test_sta_absx<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA2,
            0x7B,
        ],
    );
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xA7]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x9D]);
    check_rw_is_low(&rw);
    usart_write(usart, &[0x80, 0x20]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFB, 0x20, 0xA7]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_sta_absx passed!");
}

pub fn test_stx_zp<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xa2,
            0xaa,
        ],
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
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA2,
            0xA0,
        ],
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
    usart
        .blocking_write(&[CpuMode::DebugWithinMockMemory as u8])
        .unwrap();

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

pub fn test_sty_zp<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xa0,
            0x2c,
        ],
    );
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x84]);
    check_rw_is_low(&rw);
    usart.blocking_write(&[0x45]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x45, 0x00, 0x2C]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_sty_zp passed!");
}

pub fn test_sty_abs<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA0,
            0x44,
        ],
    );
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8C]);
    check_rw_is_low(&rw);
    usart_write(usart, &[0x00, 0xA9]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x00, 0xA9, 0x44]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_sty_zpx passed!");
}

pub fn test_sty_zpx<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA2,
            0xCC,
        ],
    );
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x90]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x94]);
    check_rw_is_low(&rw);
    usart.blocking_write(&[0x45]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x11, 0x00, 0x90]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_sty_zpx passed!");
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
    test_sta_abs(&mut usart, &nop, &rw, &mut resb);
    test_sta_indy(&mut usart, &nop, &rw, &mut resb);
    test_sta_zpx(&mut usart, &nop, &rw, &mut resb);
    test_sta_absy(&mut usart, &nop, &rw, &mut resb);
    test_sta_absx(&mut usart, &nop, &rw, &mut resb);
    test_stx_zp(&mut usart, &nop, &rw, &mut resb);
    test_stx_abs(&mut usart, &nop, &rw, &mut resb);
    test_stx_zpy(&mut usart, &nop, &rw, &mut resb);
    test_sty_zp(&mut usart, &nop, &rw, &mut resb);
    test_sty_abs(&mut usart, &nop, &rw, &mut resb);
    test_sty_zpx(&mut usart, &nop, &rw, &mut resb);
    info!("all tests passed!");
    loop {}
}
