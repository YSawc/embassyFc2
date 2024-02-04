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
    exe_testcase("jmp_abs_test".to_string(), 8);
}
