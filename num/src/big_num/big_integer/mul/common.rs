use std::{fs, path::PathBuf};

// pub const MUL_RESULT_PATH: &str = "../../../../tests/data/12345678-rep-2048-mul-result.txt";
pub(crate) const MUL_RESULT_PATH: &str =
    "F:/Develop/rust/mylib-rs/num/tests/data/12345678-rep-2048-mul-result.txt";

pub(crate) fn assert_res(actual: &str, expect_result_path: &str) {
    let expect = read_expected_result(expect_result_path);
    if actual != expect {
        panic!(
            "Results don't match!\n\
             Expected length: {}\n\
             Actual length: {}\n\
             Expected first 100 chars: {}\n\
             Actual first 100 chars: {}\n\
             Expected last 100 chars: {}\n\
             Actual last 100 chars: {}",
            expect.len(),
            actual.len(),
            &expect[..expect.len().min(100)],
            &actual[..actual.len().min(100)],
            &expect[expect.len().saturating_sub(100)..],
            &actual[actual.len().saturating_sub(100)..]
        );
    }
}

fn read_expected_result(result_path: &str) -> String {
    let path = PathBuf::from(result_path);
    if !path.exists() {
        panic!("Result file not exists: {}", result_path);
    }

    fs::read_to_string(&path)
        .unwrap_or_else(|e| {
            panic!(
                "Failed to read expected result file: {}\nPath: {:?}",
                e, path
            )
        })
        .trim()
        .to_string()
}
