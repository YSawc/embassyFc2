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
    write_buf1[2] = 0x6c;
    write_buf1[3] = 0x00;
    write_buf1[4] = 0x02;
    unwrap!(usart.blocking_write(&write_buf1));
    // let mut read_buf = [0x0u8; 2];
    let mut read_buf = [0x0u8; 1];
    info!("wrote 0x6c instruction and imm data.");
    'access_memory: loop {
        match usart.blocking_read(&mut read_buf) {
            Ok(_) => {
                match read_buf {
                    [0x00] => {
                        // [0xf5, 0xc5] => {
                        // [0x00, 0x02] => {
                        info!("6502 access valid memory.");
                        break 'access_memory;
                    }
                    v => info!("test failed. return value is {:?}", v),
                }
                info!("wait kill..");
                loop {}
            }
            Err(_) => (),
        }
    }

    let mut mock_memory_return = [0x7e, 0xdb];
    let mut read_buf2 = [0x0u8; 1];
    // let mut read_buf2 = [0x0u8; 2];
    unwrap!(usart.blocking_write(&mut mock_memory_return));
    loop {
        match usart.blocking_read(&mut read_buf2) {
            Ok(_) => {
                match read_buf2 {
                    [0xdb] => {
                        // [0x80, 0x6e] => {
                        info!("test passed!");
                    }
                    v => {
                        info!("test failed. return value is {:?}", v);
                    }
                }
                info!("wait kill..");
                loop {}
            }
            Err(_) => (),
        }
    }
}
