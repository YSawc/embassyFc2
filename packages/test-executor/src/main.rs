use std::time::Duration;
use std::{
    env,
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
};
use tokio::time::timeout;

async fn exe_testcase(testcase: String) {
    let mut test_process = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(&testcase)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let reader = BufReader::new(test_process.stdout.as_mut().unwrap());
    let line = reader
        .lines()
        .map_while(Result::ok)
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
    let root = Path::new("../stm32l476rg/src/bin");
    env::set_current_dir(root).unwrap();
    let stdout = Command::new("ls").output().unwrap().stdout;
    let raw_stdout = String::from_utf8_lossy(&stdout);
    let testcases: Vec<&str> = raw_stdout
        .split(".rs\n")
        .filter(|testcase| !testcase.is_empty())
        .collect();
    let root = Path::new("../");
    env::set_current_dir(root).unwrap();

    let timelimit = Duration::from_secs(13);
    for testcase in testcases {
        timeout(timelimit, exe_testcase(testcase.to_string()))
            .await
            .unwrap();
    }
    println!("all test passed.");
}
