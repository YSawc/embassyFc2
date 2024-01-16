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
    let mut write_buf1 = [0x0u8; 5];
    write_buf1[0] = CpuMode::Debug as u8;
    write_buf1[1] = OpeMode::Inst as u8;
    write_buf1[2] = 0x4c;
    write_buf1[3] = 0xf5;
    write_buf1[4] = 0xc5;
    let mut mock_memory = [0x0u8; 0xffff];
    mock_memory[0xc5f5] = 0x6e;
    unwrap!(usart.blocking_write(&write_buf1));
    let mut read_buf = [0x0u8; 2];
    info!("wrote 0xa9 instruction and imm data.");
    loop {
        match usart.blocking_read(&mut read_buf) {
            Ok(_) => {
                match read_buf {
                    [0xf5, 0xc5] => info!("test passed!"),
                    v => info!("test failed. return value is {:?}", v),
                }
                info!("wait kill..");
                loop {}
            }
            Err(_) => (),
        }
    }
}
