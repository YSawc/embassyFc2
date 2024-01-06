#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::Mode;
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
    let mut buf = [0x0u8; 2];
    buf[0] = Mode::Callback as u8;
    buf[1] = 0xba;
    unwrap!(usart.blocking_write(&buf));
    info!("wrote mode and additional data.");
    loop {
        match usart.blocking_read(&mut buf) {
            Ok(_) => {
                match buf.first().unwrap() {
                    0xba => info!("test passed!"),
                    v => info!(
                        "callback received but not expected value returned! return value is {:?}",
                        v
                    ),
                }
                info!("wait kill..");
                loop {}
            }
            Err(_) => (),
        }
    }
}
