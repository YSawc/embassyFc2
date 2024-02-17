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

pub fn test_inc_abs_x_without_carry<T: BasicInstance, P: Pin>(usart: &mut Uart<T>, nop: &Input<P>) {
    send_reset_signal_if_not_nop(usart, &nop);
    usart.blocking_write(&[CpuMode::Debug as u8]).unwrap();

    // store 0x30 to x
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0x30]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xFE, 0x30, 0x40]);
    let mut read_buf = [0x0u8; 2];
    usart.blocking_read(&mut read_buf).unwrap();
    match read_buf {
        [0x60, 0x40] => info!("6502 access valid memory!"),
        v => {
            info!("test failed. return value is {:?}", v);
            loop {}
        }
    }
    usart_write(usart, &[0x60]);
    let mut data_buf = [0x0u8; 1];
    usart.blocking_read(&mut data_buf).unwrap();
    match data_buf {
        [0x61] => info!("receive valid data."),
        v => {
            info!("test failed. return value is {:?}", v);
            loop {}
        }
    }
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
}

pub fn test_inc_abs_x_with_carry<T: BasicInstance, P: Pin>(usart: &mut Uart<T>, nop: &Input<P>) {
    send_reset_signal_if_not_nop(usart, &nop);
    usart.blocking_write(&[CpuMode::Debug as u8]).unwrap();

    // store 0xff to x
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0xff]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xFE, 0x30, 0x40]);
    let mut read_buf = [0x0u8; 2];
    usart.blocking_read(&mut read_buf).unwrap();
    match read_buf {
        [0x2F, 0x41] => info!("6502 access valid memory!"),
        v => {
            info!("test failed. return value is {:?}", v);
            loop {}
        }
    }
    usart_write(usart, &[0x60]);
    let mut data_buf = [0x0u8; 1];
    usart.blocking_read(&mut data_buf).unwrap();
    match data_buf {
        [0x61] => info!("receive valid data."),
        v => {
            info!("test failed. return value is {:?}", v);
            loop {}
        }
    }
    check_valid_register_status(usart, TxReg::P, &[0b00000001]);
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
    test_inc_abs_x_without_carry(&mut usart, &nop);
    test_inc_abs_x_with_carry(&mut usart, &nop);

    info!("test passed!");
    loop {}
}
