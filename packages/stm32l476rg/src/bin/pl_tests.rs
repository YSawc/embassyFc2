#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::*;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
use embassy_stm32::usart::{BasicInstance, Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use stm32l476rg::pin::util::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub fn test_plp_impl_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    check_valid_register_status(usart, TxReg::S, &[0xFE]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFE, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0b00000011]);
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x28]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFE, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0b00000011]);
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
    info!("test_plp_impl_within_internal_memory passed!");
}

pub fn test_pla_impl_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    check_valid_register_status(usart, TxReg::S, &[0xFE]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFE, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    check_valid_register_status(usart, TxReg::P, &[0b10000001]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x68]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    check_valid_register_status(usart, TxReg::P, &[0b10000001]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFE, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    check_valid_register_status(usart, TxReg::P, &[0b10000001]);
    info!("test_pla_impl_within_internal_memory passed!");
}

pub fn test_plp_impl_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x82]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFE, 0x01, 0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFE]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x28]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFE, 0x01]);
    usart.blocking_write(&[0b00000011]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    info!("test_plp_impl_within_mocking_memory passed!");
}

pub fn test_pla_impl_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x82]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFE, 0x01, 0x82]);
    check_valid_register_status(usart, TxReg::S, &[0xFE]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x68]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFE, 0x01]);
    usart.blocking_write(&[0x82]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    check_valid_register_status(usart, TxReg::P, &[0b10000001]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    info!("test_pla_impl_within_mocking_memory passed!");
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
    test_plp_impl_within_internal_memory(&mut usart, &nop, &mut resb);
    test_pla_impl_within_internal_memory(&mut usart, &nop, &mut resb);

    test_plp_impl_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_pla_impl_within_mocking_memory(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
