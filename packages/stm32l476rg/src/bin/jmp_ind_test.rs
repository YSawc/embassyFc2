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
    buf[0] = 0x6c;
    'blocking_write_instruction: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write instruction.");
                break 'blocking_write_instruction;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }
    buf[0] = 0x00;
    'blocking_write_ind_low: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write ind low data.");
                break 'blocking_write_ind_low;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }
    buf[0] = 0x02;
    'blocking_write_ind_high: loop {
        match usart.blocking_write(&buf) {
            Ok(_) => {
                info!("write ind high data.");
                break 'blocking_write_ind_high;
            }
            Err(e) => info!("error while writing: {}", e),
        }
    }

    let mut read_buf = [0x0u8; 2];
    info!("wrote 0x6c instruction and imm data.");
    'access_memory: loop {
        match usart.blocking_read(&mut read_buf) {
            Ok(_) => {
                match read_buf {
                    [0x00, 0x02] => {
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
    loop {}

    // buf[0] = 0x7e;
    // 'blocking_write_memory_low: loop {
    //     match usart.blocking_write(&buf) {
    //         Ok(_) => {
    //             info!("write memory low data.");
    //             break 'blocking_write_memory_low;
    //         }
    //         Err(e) => info!("error while writing: {}", e),
    //     }
    // }

    // buf[0] = 0xdb;
    // 'blocking_write_memory_high: loop {
    //     match usart.blocking_write(&buf) {
    //         Ok(_) => {
    //             info!("write memory high data.");
    //             break 'blocking_write_memory_high;
    //         }
    //         Err(e) => info!("error while writing: {}", e),
    //     }
    // }

    // let mut read_buf2 = [0x0u8; 1];
    // // let mut read_buf2 = [0x0u8; 2];
    // loop {
    //     match usart.blocking_read(&mut read_buf2) {
    //         Ok(_) => {
    //             match read_buf2 {
    //                 [0xdb] => {
    //                     // [0x7e, 0xdb] => {
    //                     info!("test passed!");
    //                 }
    //                 v => {
    //                     info!("test failed. return value is {:?}", v);
    //                 }
    //             }
    //             info!("wait kill..");
    //             loop {}
    //         }
    //         Err(_) => (),
    //     }
    // }
}
