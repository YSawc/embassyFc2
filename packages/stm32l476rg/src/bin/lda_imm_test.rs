#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode};
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART4 => usart::InterruptHandler<peripherals::UART4>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let config = Config::default();
    let mut usart = Uart::new(p.UART4, p.PA1, p.PA0, Irqs, NoDma, NoDma, config).unwrap();
    let mut buf = [0x0u8; 4];
    buf[0] = CpuMode::Debug as u8;
    buf[1] = OpeMode::Inst as u8;
    buf[2] = 0xa9;
    buf[3] = 0x34;
    unwrap!(usart.blocking_write(&buf));
    info!("wrote mode and additional data.");
    loop {
        match usart.blocking_read(&mut buf) {
            Ok(_) => {
                match buf.first().unwrap() {
                    0x34 => info!("test passed!"),
                    v => info!("test failed. return value is {:?}", v),
                }
                info!("wait kill..");
                loop {}
            }
            Err(_) => (),
        }
    }
}
