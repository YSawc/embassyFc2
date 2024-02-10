use defmt::*;
use embassy_fc2_app::middleware::mode::{OpeMode, TxReg};
use embassy_stm32::usart::{BasicInstance, Uart};

pub fn check_valid_p_status<T: BasicInstance>(usart: &mut Uart<T>, expect_data: &[u8]) {
    usart
        .blocking_write(&[OpeMode::RegisterTransfer as u8])
        .unwrap();
    info!("write operation mode.");
    usart.blocking_write(&[TxReg::P as u8]).unwrap();
    info!("write tx reg.");
    let mut read_buf = [0x0u8; 1];
    usart.blocking_read(&mut read_buf).unwrap();
    if read_buf == expect_data {
        info!("valid p register.");
    } else {
        info!("test failed. return value is {:?}", read_buf);
        loop {}
    }
}
