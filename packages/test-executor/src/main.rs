use std::time::Duration;
use std::{
    env,
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
};
use tokio::time::timeout;

async fn exe_testcase(testcase: String) {
    let root = Path::new("../stm32l476rg/");
    assert!(env::set_current_dir(&root).is_ok());
    let mut test_process = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(format!("{}", testcase))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let reader = BufReader::new(test_process.stdout.as_mut().unwrap());
    let line = reader
        .lines()
        .filter_map(|line| line.ok())
        .find(|line| line.contains("all tests passed"));
    if line.is_some() {
        println!("{} passed.", testcase);
        test_process.kill().unwrap();
        return;
    }
    panic!("{} failed", testcase);
}

#[tokio::main]
async fn main() {
    let timelimit = Duration::from_secs(8);
    let testcases = [
        "adc_tests".to_string(),
        "cl_tests".to_string(),
        "cmp_tests".to_string(),
        "cpx_tests".to_string(),
        "cpy_tests".to_string(),
        "inc_tests".to_string(),
        "inx_impl_test".to_string(),
        "iny_impl_test".to_string(),
        "jmp_tests".to_string(),
        "lda_tests".to_string(),
        "ldx_tests".to_string(),
        "ldy_tests".to_string(),
        "sbc_tests".to_string(),
        "se_tests".to_string(),
        "sta_tests".to_string(),
        "stx_tests".to_string(),
        "sty_tests".to_string(),
    ];
    for testcase in testcases {
        timeout(timelimit, exe_testcase(testcase)).await.unwrap();
    }
    println!("all test passed.");
}
