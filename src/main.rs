use std::io;
use std::fs;
use std::io::Write;

fn main() {
    print!("Enter file to assemble.. ");
    let mut file_name = String::from("");
    io::stdin().read_line(&mut file_name).expect("Failed to read File name, please try again.");
    println!();
    let trimmed_file_name: String = String::from(file_name.trim());
    let program= match fs::read_to_string(trimmed_file_name) {
        Ok(v) => {v},
        Err(_e) => return,
    };

    let tokens: Vec<String> = program.split_whitespace().map(String::from).collect();
    let mut output: Vec<u8> = Vec::new();

    //actually assembling the code starts here
    for token in tokens {
        if let Ok(number) = u8::from_str_radix(&token, 16) {
            output.push(number);
        } else {
            let str: &str = &token;
            match str {
                "HLT" => output.push(01),
                "ADD" => output.push(02),
                "SUB" => output.push(03),
                "LOAD" => output.push(04),
                "LOADM" => output.push(05),
                "READM" => output.push(06),
                "JMP" => output.push(07),
                "CMP" => output.push(08),
                "JE" => output.push(09),
                "R0" => output.push(00),
                "R1" => output.push(01),
                "R2" => output.push(02),
                "R3" => output.push(03),
                "R4" => output.push(04),
                "R5" => output.push(05),
                "R6" => output.push(06),
                "R7" => output.push(07),
                _ => return
            }
        }
    }

    for number in &output {
        print!("{:02X} ", number);
    }

    let file = fs::File::create("output.bin");
    let mut result_file: fs::File = match file {
        Ok(o) => o,
        Err(err) => return
    };

    result_file.write_all(&output);
    
}