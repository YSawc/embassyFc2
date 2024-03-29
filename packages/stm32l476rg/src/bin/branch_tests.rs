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

pub fn test_bpl_rel_condition_neg_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xB2, 0xC7]);
    check_valid_register_status(usart, TxReg::PC, &[0xB2, 0xC7]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x10, 0x04]);
    check_valid_register_status(usart, TxReg::PC, &[0xB8, 0xC7]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_bpl_rel_condition_neg_n passed!");
}

pub fn test_bpl_rel_condition_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x80]);
    check_valid_register_status(usart, TxReg::X, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xBB, 0xC7]);
    check_valid_register_status(usart, TxReg::PC, &[0xBB, 0xC7]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x10, 0x03]);
    check_valid_register_status(usart, TxReg::PC, &[0xBD, 0xC7]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_bpl_rel_condition_n passed!");
}

pub fn test_bmi_rel_condition_neg_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0x3C, 0xC8]);
    check_valid_register_status(usart, TxReg::PC, &[0x3C, 0xC8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x30, 0x07]);
    check_valid_register_status(usart, TxReg::PC, &[0x3E, 0xC8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_bmi_rel_condition_neg_n passed!");
}

pub fn test_bmi_rel_condition_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x80]);
    check_valid_register_status(usart, TxReg::X, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0x9B, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0x9B, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x30, 0x02]);
    check_valid_register_status(usart, TxReg::PC, &[0x9F, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);

    info!("test_bmi_rel_condition_n passed!");
}

pub fn test_bvc_rel_condition_neg_v_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xC5, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xC5, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x50, 0x02]);
    check_valid_register_status(usart, TxReg::PC, &[0xC9, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_bvc_rel_condition_neg_v passed!");
}

pub fn test_bvc_rel_condition_v_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x69, 0x1]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x80]);
    check_valid_register_status(usart, TxReg::X, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xA9, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xA9, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x50, 0x04]);
    check_valid_register_status(usart, TxReg::PC, &[0xAB, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);

    info!("test_bvc_rel_condition_v passed!");
}

pub fn test_bvs_rel_condition_neg_v_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xEE, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xEE, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x70, 0x04]);
    check_valid_register_status(usart, TxReg::PC, &[0xF0, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_bvs_rel_condition_neg_v passed!");
}

pub fn test_bvs_rel_condition_v_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x69, 0x1]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x80]);
    check_valid_register_status(usart, TxReg::X, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x70, 0x02]);
    check_valid_register_status(usart, TxReg::PC, &[0xE2, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b11000000]);

    info!("test_bvs_rel_condition_v passed!");
}

pub fn test_bcc_rel_condition_neg_c_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xC5, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xC5, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x90, 0x02]);
    check_valid_register_status(usart, TxReg::PC, &[0xC9, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_bcc_rel_condition_neg_c passed!");
}

pub fn test_bcc_rel_condition_c_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xA9, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xA9, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x90, 0x04]);
    check_valid_register_status(usart, TxReg::PC, &[0xAB, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);

    info!("test_bcc_rel_condition_c passed!");
}

pub fn test_bcs_rel_condition_neg_c_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB0, 0x02]);
    check_valid_register_status(usart, TxReg::PC, &[0xE0, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_bcs_rel_condition_neg_c passed!");
}

pub fn test_bcs_rel_condition_c_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB0, 0x02]);
    check_valid_register_status(usart, TxReg::PC, &[0xE2, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    info!("test_bcs_rel_condition_c passed!");
}

pub fn test_bne_rel_condition_neg_z_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xC5, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xC5, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xD0, 0x02]);
    check_valid_register_status(usart, TxReg::PC, &[0xC9, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_bne_rel_condition_neg_z passed!");
}

pub fn test_bne_rel_condition_z_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x18]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xA9, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xA9, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xD0, 0x04]);
    check_valid_register_status(usart, TxReg::PC, &[0xAB, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);

    info!("test_bne_rel_condition_z passed!");
}

pub fn test_beq_rel_condition_neg_z_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xF0, 0x02]);
    check_valid_register_status(usart, TxReg::PC, &[0xE0, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_beq_rel_condition_neg_z passed!");
}

pub fn test_beq_rel_condition_z_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x18]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::PC, &[0xDE, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xF0, 0x02]);
    check_valid_register_status(usart, TxReg::PC, &[0xE2, 0xD9]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    info!("test_beq_rel_condition_z passed!");
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
    test_bpl_rel_condition_neg_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bpl_rel_condition_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bmi_rel_condition_neg_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bmi_rel_condition_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bvc_rel_condition_neg_v_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bvc_rel_condition_v_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bvs_rel_condition_neg_v_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bvs_rel_condition_v_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bcc_rel_condition_neg_c_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bcc_rel_condition_c_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bcs_rel_condition_neg_c_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bcs_rel_condition_c_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bne_rel_condition_neg_z_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_bne_rel_condition_z_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_beq_rel_condition_neg_z_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_beq_rel_condition_z_within_mocking_memory(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
