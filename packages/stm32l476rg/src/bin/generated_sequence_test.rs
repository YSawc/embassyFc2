#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::*;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
use embassy_stm32::usart::{BasicInstance, Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use stm32l476rg::pin::util::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub fn jmp_c000<T: BasicInstance>(usart: &mut Uart<T>) {
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0x00, 0xC0]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    check_valid_register_status(usart, TxReg::PC, &[0x00, 0xC0]);
}

pub fn test_inst_sequence<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::NesTest as u8]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    jmp_c000(usart);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);
    usart_write(usart, &[OpeMode::Sequence as u8, 200]);

    // step to 5260
    usart_write(usart, &[OpeMode::Sequence as u8, 59]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::X, &[0x03]);
    check_valid_register_status(usart, TxReg::Y, &[0x77]);
    check_valid_register_status(usart, TxReg::P, &[0x67]);
    check_valid_register_status(usart, TxReg::S, &[0xFB]);
    check_valid_register_status(usart, TxReg::PC, &[0x45, 0xE5]);

    info!("test_inst_sequence passed!");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let config = Config::default();
    let mut usart = Uart::new_with_rtscts(
        p.USART1, p.PA10, p.PA9, Irqs, p.PA12, p.PA11, NoDma, NoDma, config,
    )
    .unwrap();
    let _rw = Input::new(p.PA0, Pull::None);
    let nop = Input::new(p.PA1, Pull::None);
    let mut resb = Output::new(p.PA4, Level::Low, Speed::Medium);
    test_inst_sequence(&mut usart, &nop, &mut resb);

    info!("all tests passed!");
    loop {}
}
