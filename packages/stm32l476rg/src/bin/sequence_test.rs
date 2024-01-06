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
    unwrap!(usart.blocking_write(&[Mode::Sequence as u8, 1]));
    info!("wrote mode");
    let mut c = 100000;
    while c >= 0 {
        c -= 1;
    }
    unwrap!(usart.blocking_write(&[3 as u8, 1]));
    info!("wrote sequence count");
    loop {}
}
