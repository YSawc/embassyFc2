#![no_std]
#![no_main]

use defmt::*;
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
    enum Mode {
        Stop,
        Normal = 2,
        Callback = 3,
        Sequence = 0xFE,
    }
    unwrap!(usart.blocking_write(&[Mode::Callback as u8, 1]));
    info!("wrote mode");

    let mut buf = [0x0u8; 1];
    loop {
        match usart.blocking_read(&mut buf) {
            Ok(_) => {
                match buf.first().unwrap() {
                    3 => info!("test passed!"),
                    _ => info!("callback received but not expected value returned!"),
                }
                info!("wait kill..");
                loop {}
            }
            Err(_) => (),
        }
    }
}
