use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::fs::rename;
use std::io::BufRead;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    #[clap(short, long)]
    outfile: String,

    #[clap(short, long)]
    size: Option<String>,

    #[clap(short, long)]
    keep: Option<i32>,

    #[clap(short, long)]
    dated: Option<bool>,

    #[clap(short, long)]
    buffered: Option<bool>,
}

fn main() {
    println!("{}", date());
    let args = Cli::parse();
    let size_string = args.size.unwrap_or("100MiB".to_string());
    let size = calc_size(size_string.as_str());
    if let Ok(size_number) = size  {
        let run_result = run(args.outfile, size_number,args.keep.unwrap_or(20), args.dated.unwrap_or(false), 
            args.buffered.unwrap_or(false));
        if let Ok(how_many_bytes) = run_result {
            println!("{} bytes copied.", how_many_bytes);
        } else if let Err(why) = run_result {
            println!("Error {:?}", why);
        }
    } else {
        println!("Invalid size: {}", size_string);
    }
    //print!("Argument is {:?}, {}", args, flag);
}

fn date() -> String {
    use chrono::prelude::*;
    let date_as_string = Local::now().to_string();
    return date_as_string;
}

fn run(outfile:String, size: i64, keep: i32, dated: bool, buffered: bool) -> Result<i64, io::Error>{
    let mut bytes_written:i64 = 0;
    let mut buffer = [0 as u8; 256 * 1024];
    let mut total_written = 0;
    let mut stdin = io::stdin(); // We get `Stdin` here.
    let mut fh = reopen(outfile.as_str(), &mut bytes_written, size, keep);
    loop {
        let nread = stdin.read(&mut buffer);
        if let Err(why) = nread  {
            return Err(io::Error::new(ErrorKind::BrokenPipe, why));
        }
        let nread_i = nread.unwrap();
        if nread_i == 0 {
            // EOF
            return Ok(total_written);
        }

        
        let mut buf = String::new();
        let mut pointer = &buffer[0..nread_i];
        let string_ok = std::str::from_utf8(&buffer).is_ok();
        let mut date_prefix = String::new();
        if dated {
            date_prefix = String::new() + "[" + date().as_str() + "] ";
        }
        let date_bytes = date_prefix.as_bytes();
        if string_ok {
            loop {
                let nread = pointer.read_line(&mut buf);
                let nread_i = nread.unwrap();
                if nread_i == 0 {
                    // EOF
                    break;
                }

                let future_size = bytes_written + nread_i as i64 + date_prefix.len() as i64;
                if future_size > size {
                    // this write will exceed
                    if bytes_written > 0 {
                        // only rotate if the bytes written is greater than 0, to avoid rotating on empty file
                        drop(fh);
                        rotate(outfile.as_str(), keep);
                        fh = reopen(outfile.as_str(), &mut bytes_written, size, keep);
                    }
                }

                let to_write = buf.as_bytes();
                if date_bytes.len() > 0 {
                    fh.write(date_bytes).expect("Failed to write to output file");
                }
                fh.write(to_write).expect("Failed to write to output file");
                total_written += (nread_i as i64 + date_bytes.len() as i64);
                bytes_written += (nread_i as i64 + date_bytes.len() as i64);
                // write data
                buf.clear();
            }
        } else {
            // pinter may still has some data
            let mut new_buffer = [0 as u8; 256*1024];
            let nread = pointer.read(&mut new_buffer);
            // this will not fail most likely
            let nread_i = nread.unwrap();
            if nread_i > 0 {
                let mut future_size = bytes_written + nread_i as i64 + date_bytes.len() as i64;
                if dated {
                    future_size+=1;
                }
                if future_size > size {
                    // this write will exceed
                    if bytes_written > 0 {
                        // only rotate if the bytes written is greater than 0, to avoid rotating on empty file
                        drop(fh);
                        rotate(outfile.as_str(), keep);
                        fh = reopen(outfile.as_str(), &mut bytes_written, size, keep);
                    }
                }
                if date_bytes.len() > 0 {
                    fh.write(date_bytes).expect("Failed to write to output file");
                }
                fh.write(&buffer[0..nread_i]).expect("Failed to write to output file");
                if date_bytes.len() > 0 {
                    fh.write("\n".as_bytes()).expect("Failed to write to output file");
                }
                total_written += nread_i as i64 + date_bytes.len() as i64;
                bytes_written += nread_i as i64 + date_bytes.len() as i64;
            }
            // the string was ok actually, very unlikely
        }
        if !buffered {
            if let Err(why) = fh.flush() {
                return Err(io::Error::new(io::ErrorKind::Other, why));
            }
        }
        // process it
    }
}

fn reopen(outfile:&str, size: &mut i64, size_limit:i64, keep: i32) ->File {
    let fh = File::options().create(true).append(true).open(outfile).expect("Failed to open file");
    *size = fh.metadata().unwrap().len() as i64;
    if *size > size_limit {
        println!("Rotating due to file open size exceeding.");
        // File is already too big
        drop(fh);
        rotate(outfile, keep);
        return reopen(outfile, size, size_limit, keep);
    }
    return fh;
}

fn rotate(outfile:&str, keep:i32) {
    for i in (1..keep).rev() {
        let me_path = format!("{}.{}", outfile, i);
        let me = Path::new(me_path.as_str());
        let next_path = format!("{}.{}", outfile, i + 1);
        let next = Path::new(next_path.as_str());
        if me.exists() {
            rename(me, next)
                .expect(format!("Failed to rename {} to {}", me.display(), next.display()).as_str());
        }
    }

    let me = Path::new(outfile);
    let me_next_path = format!("{}.1", outfile);
    let next = Path::new(me_next_path.as_str());

    if me.exists()  {
        rename(me, next).expect(format!("Failed to rename {} to {}", me.display(), next.display()).as_str());
    }
}
#[derive(Debug)]
pub enum ArgumentError {
    InvalidPattern,
    NotNumber,
    InvalidUnit
}

fn calculate_by_unit(count: &str, unit: &str) -> Result<i64, ArgumentError> {
    let lookup = HashMap::from([
            ("k", 1000),
            ("K" , 1024),
            ("m" , 1000000),
            ("M" , 1048576),
            ("g" , 1000000000),
            ("G" , 1073741824),
            ("kB" , 1000),
            ("KB" , 1024),
            ("KiB" , 1024),
            ("mB" , 1000000),
            ("MB" , 1048576),
            ("MiB" , 1048576),
            ("gB" , 1000000000),
            ("GB" , 1073741824),
            ("GiB" , 1073741824)
        ]);

    let unit_size:i32;
    if unit == "" {
        unit_size = 1;
    } else {
        let lookup_result = lookup.get(unit);
        match lookup_result {
            Some(number) => unit_size = *number,
            _ => return Err(ArgumentError::InvalidUnit)
        }
    }
    let count_result = count.parse::<i32>();
    match count_result {
        Ok(number) => return Ok(number as i64 * unit_size as i64),
        Err(_) => return Err(ArgumentError::NotNumber)
    }
}

fn calc_size(input:&str) -> Result<i64, ArgumentError> {
    let re = Regex::new(r"[_,]").unwrap();
    let result = re.replace_all(input, "");
    let re_match = Regex::new(r"^(\d+)(\D+)?$").unwrap();
    let caps = re_match.captures(&result);
    return match caps {
        Some(group) => {
            let count = group.get(1).unwrap().as_str();
            let unit_raw = group.get(2);
            let unit = match unit_raw {
                Some(what) => what.as_str(),
                _ => "",
            };
            calculate_by_unit(count, unit)
        },
        _ => {
            println!("{}", result);
            Err(ArgumentError::InvalidPattern)
        }
    }
}