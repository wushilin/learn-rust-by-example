use std::fmt;
use std::io;
use std::io::Write;

// The max number for fibonacci
const MAX:i32 = 50;


fn main() {
    let mut memory:[i64;(MAX + 1) as usize] = [-1; (MAX + 1) as usize];
    loop {
        print!("Enter a number (quit to end) > ");
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        let read_result = io::stdin().read_line(&mut buffer);
        match read_result {
            Err(e) => {
                print!("Failed to read what you said. Please try again! ({})\n", e);
                continue;
            },
            Ok(_) => ()
        }
        if buffer.trim().eq("quit") {
            print!("Bye!\n");
            break;
        }
        let _ = io::stdout().flush();
        let parsed_result = buffer.trim().parse::<i32>();
        let mut parsed:i32;
        match parsed_result {
            Ok(i) => parsed = i,
            Err(e) => {
                print!("Invalid input: {} ({})\n", buffer.trim(), e);
                continue;
            }
        }
        if parsed < 0 {
            print!("Invalid input: {} (<0) \n", parsed);
            continue;
        }
        let result = fibonacci(parsed, &mut memory);
        match result {
            Ok(i) => print!("fibonacci({}) = {}\n", parsed, i),
            Err(e) => print!("Invalid input: {}\n", e)
        }
    }
}

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct ParamError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The Input Parameter is out of range!")
    }
}

fn fibonacci(target:i32, memory:&mut [i64;(MAX + 1) as usize]) -> Result<i64, ParamError> {
    if target > MAX {
        let err = ParamError{};
        return Err(err);
    }
    if memory[target as usize] != -1 {
        return Ok(memory[target as usize] as i64);
    }

    if target <= 1 {
        memory[target as usize] = 1;
        return Ok(1);
    }

    let result1 = fibonacci(target - 2, memory);
    let result2 = fibonacci(target - 1, memory);
    let result = result1.expect("Safe!") + result2.expect("Safe!");
    memory[target as usize] = result;
    return Ok(result);
}