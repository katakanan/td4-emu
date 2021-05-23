use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::u8;

#[derive(Debug, Default)]
pub struct Mem {
    pub mem: Vec<u8>,
}

impl Mem {
    pub fn load_new(path: &str) -> Mem {
        let mut mem = Mem::default();
        mem.load(path);
        mem
    }

    fn load(&mut self, path: &str) {
        let lines = read_lines(path).unwrap_or_else(|_| panic!());
        for line in lines {
            if let Ok(bin_str) = line {
                let bin = u8::from_str_radix(&bin_str, 2).unwrap_or_else(|_| 0xAA);
                self.mem.push(bin);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
