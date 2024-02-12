use std::{env, path::Path, process::Command};

fn exe_testcase(testcase: String, timeout: u8) {
    let root = Path::new("../stm32l476rg/");
    assert!(env::set_current_dir(&root).is_ok());
    match Command::new("timeout")
        .arg(format!("{}", timeout))
        .arg("cargo")
        .arg("run")
        .arg("--bin")
        .arg(format!("{}", testcase))
        .output()
    {
        Ok(output) => {
            if !std::str::from_utf8(&output.stdout)
                .unwrap()
                .contains("test pass")
            {
                panic!("{} failed.", testcase);
            }
        }
        Err(e) => println!("child error occured. {}", e),
    }
    println!("{} passed.", testcase);
}

fn main() {
    exe_testcase("jmp_abs_test".to_string(), 6);
    exe_testcase("jmp_ind_test".to_string(), 6);
    exe_testcase("lda_zp_test".to_string(), 6);
    exe_testcase("lda_zpx_test".to_string(), 6);
    exe_testcase("ldx_imm_test".to_string(), 6);
    exe_testcase("ldx_zp_test".to_string(), 6);
    exe_testcase("ldx_imm_test".to_string(), 6);
    exe_testcase("ldx_zp_test".to_string(), 6);
    exe_testcase("ldx_zpy_test".to_string(), 6);
    exe_testcase("ldy_imm_test".to_string(), 6);
    exe_testcase("ldy_zp_test".to_string(), 6);
    exe_testcase("sta_zp_test".to_string(), 6);
    exe_testcase("sta_zpx_test".to_string(), 6);
    exe_testcase("stx_zp_test".to_string(), 6);
    exe_testcase("stx_zpy_test".to_string(), 6);
    exe_testcase("sty_zp_test".to_string(), 6);
    exe_testcase("inc_zp_test".to_string(), 6);
    println!("all test passed.");
}
