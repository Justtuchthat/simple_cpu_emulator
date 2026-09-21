pub mod cpu;

use std::env;
use std::fs;
use json_rw::json_parser::parse_json;
use json_rw::json_writer::write_json;

use crate::cpu::Cpu;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Too little arguments!");
    }
    let in_path = &args[1];
    let out_path = &args[2];

    let json_string = fs::read_to_string(in_path).unwrap();
    let cpu_json = parse_json(json_string).unwrap();
    let cpu = Cpu::from_json(cpu_json).unwrap();
    let cpu_json = cpu.to_json();
    let json_string = write_json(cpu_json);
    fs::write(out_path, json_string).unwrap()
}
