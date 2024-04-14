use std::{
    env,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

#[derive(Debug, Clone)]
struct RegisterStr {
    pc_high_str: String,
    pc_low_str: String,
    a_str: String,
    x_str: String,
    y_str: String,
    p_str: String,
    s_str: String,
}

impl RegisterStr {
    fn create_register_str(target_line_number: u32) -> Self {
        let file_path = "../../dump_logs/nestest.log";
        let binding = read_to_string(file_path).expect("File path need.");
        let mut read_lines = binding.lines();
        let target_line = read_lines.nth((target_line_number - 1) as usize);

        fn gen_reg_str(o_str: Option<&str>, range_s: usize, range_e: usize) -> String {
            o_str.unwrap()[range_s..range_e]
                .to_uppercase()
                .replace(' ', "0")
        }

        let pc_high_str = gen_reg_str(target_line, 0, 2);
        let pc_low_str = gen_reg_str(target_line, 2, 4);
        let a_str = gen_reg_str(target_line, 27, 29);
        let x_str = gen_reg_str(target_line, 32, 34);
        let y_str = gen_reg_str(target_line, 37, 39);
        let p_str = gen_reg_str(target_line, 42, 44);
        let s_str = gen_reg_str(target_line, 47, 49);

        Self {
            pc_high_str,
            pc_low_str,
            a_str,
            x_str,
            y_str,
            p_str,
            s_str,
        }
    }

    fn gen_test_sequence_file(&self, target_line_number: u32) {
        let path = Path::new("../stm32l476rg/src/bin/generated_sequence_test.rs");
        let mut file = File::create(path).unwrap();
        file.write_all(
            b"#![no_std]
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
",
        )
        .unwrap();
        for _ in 0..(target_line_number - 1) / 200 {
            file.write_all(b"    usart_write(usart, &[OpeMode::Sequence as u8, 200]);\n")
                .unwrap();
        }

        write!(
            &mut file,
            "
    // step to {}
    usart_write(usart, &[OpeMode::Sequence as u8, {}]);
    check_valid_register_status(usart, TxReg::A, &[0x{}]);
    check_valid_register_status(usart, TxReg::X, &[0x{}]);
    check_valid_register_status(usart, TxReg::Y, &[0x{}]);
    check_valid_register_status(usart, TxReg::P, &[0x{}]);
    check_valid_register_status(usart, TxReg::S, &[0x{}]);
    check_valid_register_status(usart, TxReg::PC, &[0x{}, 0x{}]);

    info!(\"test_inst_sequence passed!\");
}}
",
            target_line_number,
            (target_line_number - 1) % 200,
            self.a_str,
            self.x_str,
            self.y_str,
            self.p_str,
            self.s_str,
            self.pc_low_str,
            self.pc_high_str
        )
        .unwrap();

        file.write_all(
            b"
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

    info!(\"all tests passed!\");
    loop {}
}\n",
        )
        .unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_line_number = args[1].parse::<u32>().unwrap();
    let reg_s = RegisterStr::create_register_str(target_line_number);
    reg_s.gen_test_sequence_file(target_line_number);
}
