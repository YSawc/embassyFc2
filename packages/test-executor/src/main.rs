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
                .contains("all tests passed")
            {
                panic!("{} failed.", testcase);
            }
        }
        Err(e) => println!("child error occured. {}", e),
    }
    println!("{} passed.", testcase);
}

fn main() {
    exe_testcase("cl_tests".to_string(), 6);
    exe_testcase("cmp_tests".to_string(), 6);
    exe_testcase("cpx_tests".to_string(), 6);
    exe_testcase("cpy_tests".to_string(), 6);
    exe_testcase("inc_tests".to_string(), 6);
    exe_testcase("inx_impl_test".to_string(), 6);
    exe_testcase("iny_impl_test".to_string(), 6);
    exe_testcase("jmp_tests".to_string(), 6);
    exe_testcase("lda_tests".to_string(), 6);
    exe_testcase("ldx_tests".to_string(), 6);
    exe_testcase("ldy_tests".to_string(), 6);
    exe_testcase("se_tests".to_string(), 6);
    exe_testcase("sta_tests".to_string(), 6);
    exe_testcase("stx_tests".to_string(), 6);
    exe_testcase("sty_tests".to_string(), 6);
    println!("all test passed.");
}
