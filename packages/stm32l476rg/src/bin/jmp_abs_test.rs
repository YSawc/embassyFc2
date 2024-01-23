#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode};
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
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

    let mut buf = [0x0u8; 1];
    buf[0] = CpuMode::Debug as u8;
    'blocking_write_cpu_mode: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write cpu operation mode.");
                break 'blocking_write_cpu_mode;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }
    buf[0] = OpeMode::Inst as u8;
    'blocking_write_operation: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write operation mode.");
                break 'blocking_write_operation;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }
    buf[0] = 0x4c;
    'blocking_write_instruction: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write instruction.");
                break 'blocking_write_instruction;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }
    buf[0] = 0xf5;
    'blocking_write_row: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write row data.");
                break 'blocking_write_row;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }
    buf[0] = 0xc5;
    'blocking_write_high: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write high data.");
                break 'blocking_write_high;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }

    let mut read_buf = [0x0u8; 2];
    loop {
        match usart.blocking_read(&mut read_buf) {
            Ok(_) => {
                match read_buf {
                    [0xc5, 0xf5] => info!("test passed!"),
                    v => info!("test failed. return value is {:?}", v),
                }
                info!("wait kill..");
                loop {}
            }
            Err(_) => (),
        }
    }
}
