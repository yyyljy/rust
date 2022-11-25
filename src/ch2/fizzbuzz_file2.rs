use std::fs::File;
use std::io::Write;
fn main() {
    let filename = "fizzbuzz_file2_result.txt";
    let data = get_fizzbuzz(100);
    // 파일 생성 --- (*1)
    let mut fp = File::create(filename).unwrap();
    // 파일에 쓰기 --- (*2)
    let bytes = data.as_bytes();
    fp.write_all(bytes).unwrap();
}

// FizzBuzz 결과를 구하는 함수
fn get_fizzbuzz(max: u32) -> String {
    let mut result = String::new();
    for i in 1..=max {
        if (i % 3 == 0) && (i % 5 == 0) {
            result += "FizzBuzz\n";
        } else if i % 3 == 0 {
            result += "Fizz\n";
        } else if i % 5 == 0 {
            result += "Buzz\n";
        } else {
            result += &format!("{}\n", i);
        }
    }
    result
}