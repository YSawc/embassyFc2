#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode, TxReg};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Pin, Pull};
use embassy_stm32::usart::{BasicInstance, Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use stm32l476rg::pin::util::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub fn test_inc_zp_without_triger_of_p<T: BasicInstance, P: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
) {
    send_reset_signal_if_not_nop(usart, &nop);
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

pub fn test_inc_zp_with_over_flow_and_zero_flag<T: BasicInstance, P: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
) {
    send_reset_signal_if_not_nop(usart, &nop);
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

pub fn test_inc_zp_with_negative_flag<T: BasicInstance, P: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
) {
    send_reset_signal_if_not_nop(usart, &nop);
    usart_write(
        usart,
        &[CpuMode::Debug as u8, OpeMode::Inst as u8, 0xe6, 0x2c],
    );
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2C, 0x00]);
    usart.blocking_write(&[0x7f]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_inc_zp_with_over_flow_and_zero_flag passed!");
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
    test_inc_zp_without_triger_of_p(&mut usart, &nop);
    test_inc_zp_with_over_flow_and_zero_flag(&mut usart, &nop);
    test_inc_zp_with_negative_flag(&mut usart, &nop);
    info!("all test passed!");
    loop {}
}
