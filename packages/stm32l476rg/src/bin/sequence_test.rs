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
    buf[0] = Mode::Sequence as u8;
    buf[1] = 3;
    unwrap!(usart.blocking_write(&buf));
    info!("wrote mode and additional data.");

    loop {}
}
