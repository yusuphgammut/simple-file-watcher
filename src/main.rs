use std::{
    fs::{self},
    time::SystemTime,
};
use tokio::time::{interval, Duration};

const MAIN_SOURCE_FILE: &str = "main-source.txt";
const MAIN_RESULT_FILE: &str = "main-result.txt";
const RUNTIME_SOURCE_FILE: &str = "runtime-source.txt";
const RUNTIME_RESULT_FILE: &str = "runtime-result.txt";

async fn check_file(result_modtime: &mut SystemTime, source_file: &str, result_file: &str) {
    let source_modtime = fs::metadata(source_file).unwrap().modified().unwrap();
    if *result_modtime != source_modtime {
        let data: String = fs::read_to_string(source_file).unwrap();
        fs::write(result_file, data).unwrap();
        *result_modtime = source_modtime;
        println!("Successfully updated file {}.", result_file);
    }
}

#[tokio::main]
async fn main() {
    let mut main_result_modtime: SystemTime = SystemTime::now();
    let mut runtime_result_modtime: SystemTime = SystemTime::now();
    let mut interval = interval(Duration::from_secs(1)); // run every 1 second
    loop {
        interval.tick().await;
        check_file(&mut main_result_modtime, MAIN_SOURCE_FILE, MAIN_RESULT_FILE).await;
        check_file(
            &mut runtime_result_modtime,
            RUNTIME_SOURCE_FILE,
            RUNTIME_RESULT_FILE,
        )
        .await;
    }
}
