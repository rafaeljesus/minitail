use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::{BufRead, BufReader, Read, Result},
    thread, time,
};

// cargo run -- <file_name> number of lines: <n> follow: <true|false>
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut n: usize = 10;
    if args.len() >= 3 {
        match &args[2].parse() {
            Ok(v) => n = *v,
            Err(_) => (),
        }
    }
    let mut lines: VecDeque<String> = VecDeque::with_capacity(n);
    for line in buf_reader.by_ref().lines() {
        if lines.len() == n {
            lines.pop_front();
        }
        lines.push_back(line?);
    }
    for line in lines {
        println!("{line}");
    }

    let mut follow: bool = false;
    if args.len() >= 4 {
        match &args[3].parse() {
            Ok(v) => follow = *v,
            Err(_) => (),
        }
    }
    if !follow {
        return Ok(());
    }

    loop {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(res) => {
                if res == 0 {
                    let second = time::Duration::from_secs(1);
                    thread::sleep(second);
                    continue;
                }
                println!("{line}");
            }
            Err(_) => (),
        }
    }
}
