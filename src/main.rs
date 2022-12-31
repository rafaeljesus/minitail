use std::{collections::VecDeque, env, fs::File, io::BufRead, io::BufReader, io::Result};

// cargo run -- <file_name> <n>
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file = File::open(file_name)?;
    let buf_reader = BufReader::new(file);
    let mut n: usize = 10;
    if args.len() == 3 {
        match &args[2].parse() {
            Ok(v) => n = *v,
            Err(_) => (),
        }
    }
    let mut lines: VecDeque<String> = VecDeque::with_capacity(n);
    for line in buf_reader.lines() {
        if lines.len() == n {
            lines.pop_front();
        }
        lines.push_back(line?);
    }
    for line in lines {
        println!("{line}");
    }

    // TODO implemet follow
    // if follow then read line continuously
    Ok(())
}
