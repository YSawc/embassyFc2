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

pub fn clc_impl_test<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x38]);
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x18]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("clc_impl_test passed!");
}

pub fn cld_impl_test<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b00001000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xD8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("cld_impl_test passed!");
}

pub fn cli_impl_test<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00000100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x58]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("cli_impl_test passed!");
}

pub fn clv_impl_test<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::Debug as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);

    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xe6, 0x2c],
    );
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2C, 0x00]);
    usart.blocking_write(&[0xff]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b01000010]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xB8]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    info!("clv_impl_test passed!");
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
    clc_impl_test(&mut usart, &nop, &mut resb);
    cld_impl_test(&mut usart, &nop, &mut resb);
    cli_impl_test(&mut usart, &nop, &mut resb);
    clv_impl_test(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
