use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: md5sum <file>");
        process::exit(1);
    }

    println!("{}\t{}", check_sum(args.get(1).unwrap()), args.get(1).unwrap());
}

fn check_sum(file:&String) -> String{
    let mut f = File::open(file).expect("File Open error");
    let mut buffer = [0 as u8; 1024];
    let mut hasher = Md5::new();
    hasher.reset();
    loop {
        let read_result = f.read(&mut buffer);
        let read_count = read_result.expect("Read failed");
        hasher.input(&buffer[..read_count]);
        if read_count == 0 {
            break;
        }
    }
    let result_str = hasher.result_str();
    return result_str;
}
