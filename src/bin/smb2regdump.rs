use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};
use pmi8998_typec::smb2_regs::Register;
use pmi8998_typec::args::Args;
use clap::Parser;

fn load_dumpfile(path: PathBuf) -> Vec<Register> {
    let mut regs = Vec::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() || line.contains("XX") {
            continue;
        }
        let line = line.split(": ").collect::<Vec<_>>();
        if let Some(addr) = u16::from_str_radix(line[0], 16).ok() {
            if let Some(value) = u8::from_str_radix(line[1], 16).ok() {
                regs.push(Register { addr, value });
            }
        }
    }
    regs
}

fn main() {
    let args = Args::parse();
    let regs = load_dumpfile(args.regdump);
    for reg in regs {
        let start_addr = args.start_addr.unwrap_or(0);
        if reg.addr < start_addr {
            continue;
        }
        if let Some(length) = args.length {
            if reg.addr >= start_addr + length {
                continue;
            }
        }

        println!("{}", reg);
    }
}
