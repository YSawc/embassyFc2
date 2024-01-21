#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{AddrMode, CpuMode, OpeMode};
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
    buf[0] = OpeMode::Addr as u8;
    'blocking_write_operation: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write operation mode.");
                break 'blocking_write_operation;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }
    buf[0] = AddrMode::Imm as u8;
    'blocking_write_addressing_mode: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write addressing mode.");
                break 'blocking_write_addressing_mode;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }
    buf[0] = 0xaa;
    'blocking_write_imm_data: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write imm data.");
                break 'blocking_write_imm_data;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }

    loop {
        match usart.blocking_read(&mut buf) {
            Ok(_) => {
                match buf.first().unwrap() {
                    0xaa => info!("test passed!"),
                    v => info!("test failed. return value is {:?}", v),
                }
                info!("wait kill..");
                loop {}
            }
            Err(_) => (),
        }
    }
}
