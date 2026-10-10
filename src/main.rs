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
        Err(_e) => {println!("IATE TEH FILE");
    return}
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
                "hlt" => output.push(0x01),
                "add" => output.push(0x02),
                "sub" => output.push(0x03),
                "load" => output.push(0x04),
                "loadm" => output.push(0x05),
                "readm" => output.push(0x06),
                "jmp" => output.push(0x07),
                "cmp" => output.push(0x08),
                "je" => output.push(0x09),
                "int" => output.push(0x0A),
                "r0" => output.push(0x00),
                "r1" => output.push(0x01),
                "r2" => output.push(0x02),
                "r3" => output.push(0x03),
                "r4" => output.push(0x04),
                "r5" => output.push(0x05),
                "r6" => output.push(0x06),
                "r7" => output.push(0x07),
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