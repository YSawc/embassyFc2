use defmt::*;
use embassy_fc2_app::middleware::mode::{OpeMode, TxReg};
use embassy_stm32::gpio::Input;
use embassy_stm32::gpio::Pin;
use embassy_stm32::usart::{BasicInstance, Uart};
use embassy_time::Timer;

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

pub fn send_reset_signal_if_not_nop<T: BasicInstance, P: Pin>(usart: &mut Uart<T>, nop: &Input<P>) {
    // if fpga is not nop, send reset signal
    let mut buf = [0x0u8; 1];
    match nop.is_low() {
        true => {
            buf[0] = OpeMode::Reset as u8;
            usart.blocking_write(&buf).unwrap();
            info!("send reset signal.");
            let _ = Timer::after_millis(1500);
            match nop.is_high() {
                true => info!("fpga reset!"),
                false => {
                    info!("failed to reset fpga.");
                    loop {}
                }
            }
        }
        false => {}
    }
}

pub fn check_rw_is_high<P: Pin>(rw: Input<P>) {
    match rw.is_high() {
        true => info!("rw flag is high"),
        false => {
            info!("test failed. rw flag is not high.");
            loop {}
        }
    }
}

pub fn check_rw_is_low<P: Pin>(rw: Input<P>) {
    match rw.is_low() {
        true => info!("rw flag is low"),
        false => {
            info!("test failed. rw flag is not low.");
            loop {}
        }
    }
}
