use defmt::*;
use embassy_fc2_app::middleware::mode::{OpeMode, TxReg};
use embassy_stm32::usart::{BasicInstance, Uart};

pub fn check_valid_register_status<T: BasicInstance>(
    usart: &mut Uart<T>,
    register: TxReg,
    expect_data: &[u8],
) {
    usart
        .blocking_write(&[OpeMode::RegisterTransfer as u8])
        .unwrap();
    info!("write register transfer mode.");
    usart.blocking_write(&[register as u8]).unwrap();
    info!("write tx reg.");
    match register {
        TxReg::PC => {
            let mut read_buf = [0x0u8; 2];
            usart.blocking_read(&mut read_buf).unwrap();
            if read_buf == expect_data {
                info!("valid register status.");
            } else {
                info!("test failed. return value is {:?}", read_buf);
                loop {}
            }
        }
        _ => {
            let mut read_buf = [0x0u8; 1];
            usart.blocking_read(&mut read_buf).unwrap();
            if read_buf == expect_data {
                info!("valid register status.");
            } else {
                info!("test failed. return value is {:?}", read_buf);
                loop {}
            }
        }
    };
}
