#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::CpuMode;
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
    buf[0] = CpuMode::Callback as u8;
    usart.blocking_write(&buf).unwrap();
    info!("write cpu operation mode.");
    buf[0] = 0xba;
    usart.blocking_write(&buf).unwrap();
    info!("write callback value.");
    let mut read_buf = [0x0u8; 1];
    usart.blocking_read(&mut read_buf).unwrap();
    match read_buf {
        [0xba] => info!("test passed!"),
        v => {
            info!("test failed. return value is {:?}", v);
            loop {}
        }
    }
    loop {}
}
