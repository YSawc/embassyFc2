#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode, TxReg};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use stm32l476rg::pin::util::{
    check_rw_is_low, check_valid_register_status, send_reset_signal_if_not_nop,
};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

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
    let mut buf = [0x0u8; 1];
    send_reset_signal_if_not_nop(&mut usart, &nop);
    buf[0] = CpuMode::Debug as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write cpu operation mode.");
    buf[0] = OpeMode::Inst as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write operation mode.");
    buf[0] = 0xa0;
    usart.blocking_write(&buf).unwrap();
    info!("write instruction.");
    buf[0] = 0x2c;
    usart.blocking_write(&buf).unwrap();
    info!("write store value to y.");
    check_valid_register_status(&mut usart, TxReg::P, &[0b00000000]);
    buf[0] = OpeMode::Inst as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write operation mode.");
    buf[0] = 0x84;
    usart.blocking_write(&buf).unwrap();
    info!("write instruction.");
    check_rw_is_low(rw);
    buf[0] = 0x45;
    usart.blocking_write(&buf).unwrap();
    info!("write target address.");
    let mut read_buf = [0x0u8; 1];
    usart.blocking_read(&mut read_buf).unwrap();
    match read_buf {
        [0x45] => info!("valid memory low!"),
        v => {
            info!("test failed. return value is {:?}", v);
            loop {}
        }
    }
    let mut mock_memory = [0x0u8; 0xff];
    let mut data_buf = [0x0u8; 1];
    usart.blocking_read(&mut data_buf).unwrap();
    match data_buf {
        [0x2c] => info!("receive stored data."),
        v => {
            info!("test failed. return value is {:?}", v);
            loop {}
        }
    }
    mock_memory[read_buf[0] as usize] = data_buf[0];
    check_valid_register_status(&mut usart, TxReg::P, &[0b00000000]);
    info!("test passed!");
    loop {}
}
