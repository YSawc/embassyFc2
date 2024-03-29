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

pub fn test_cmp_indx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA9,
            0x40,
        ],
    );
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC1, 0x80]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x80, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x81, 0x00]);
    usart.blocking_write(&[0x02]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x02]);
    usart.blocking_write(&[0x41]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_cmp_indx_within_mocking_memory passed!");
}

pub fn test_cmp_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA9,
            0x80,
        ],
    );
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC5, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x7F]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
    info!("test_cmp_zp_within_mocking_memory passed!");
}

pub fn test_cmp_imm_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA9,
            0x6F,
        ],
    );
    check_valid_register_status(usart, TxReg::A, &[0x6F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x6F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    info!("test_cmp_imm_within_mocking_memory passed!");
}

pub fn test_cmp_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA9,
            0x80,
        ],
    );
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x80]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    info!("test_cmp_abs_within_mocking_memory passed!");
}

pub fn test_cmp_indy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA0,
            0x02,
        ],
    );
    check_valid_register_status(usart, TxReg::Y, &[0x02]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xD1, 0x33]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x34, 0x00]);
    usart.blocking_write(&[0x04]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x02, 0x04]);
    usart.blocking_write(&[0x40]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    info!("test_cmp_indy_within_mocking_memory passed!");
}

pub fn test_cmp_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA2,
            0x78,
        ],
    );
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xD5, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x78]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_cmp_zpx_within_mocking_memory passed!");
}

pub fn test_cmp_absy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xD9, 0x00, 0x04]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0x80]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    info!("test_cmp_absy_within_mocking_memory passed!");
}

pub fn test_cmp_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(
        usart,
        &[
            CpuMode::DebugWithinMockMemory as u8,
            OpeMode::Inst as u8,
            0xA2,
            0x78,
        ],
    );
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xDD, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x41]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_cmp_absx_within_mocking_memory passed!");
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
    test_cmp_indx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_cmp_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_cmp_imm_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_cmp_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_cmp_indy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_cmp_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_cmp_absy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_cmp_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
