pub mod registers;

use crate::cpu::registers::{Registers, RAM};
use std::collections::HashMap;
use json_rw::JSONtype;


pub struct Cpu {
    registers: Registers,
    ram: RAM,
    version: i16,
    flags: Vec<String>,
}

impl Cpu {
    const CPU_VERSION: i16 = 1i16;
    pub fn default() -> Cpu {
        Cpu {
            registers: Registers::default(),
            ram: RAM::default(),
            version: Cpu::CPU_VERSION,
            flags: Vec::new(),
        }
    }

    pub fn from_json(json: JSONtype) -> Result<Cpu, String> {
        match json {
            JSONtype::Object(json_map) => {
                // Check for required fields
                if !json_map.contains_key("CPU_VERSION") {
                    return Err(String::from("Program file does not contain required CPU_VERSION field"));
                }
                if !json_map.contains_key("RAM") {
                    return Err(String::from("Program file does not contain required RAM field"));
                }

                // Get register from json and return error if there is any
                let registers = match Registers::from_json(&json_map) {
                    Ok(registers) => registers,
                    Err(msg) => {
                        return Err(msg);
                    }
                };

                // Get ram from json and return error if there is any
                let ram = match RAM::from_json(&json_map) {
                    Ok(ram) => ram,
                    Err(msg) => {
                        return Err(msg);
                    }
                };

                let cpu_version_json = json_map.get("CPU_VERSION").unwrap();
                let cpu_version: i16 = match cpu_version_json {
                    JSONtype::Int(val) => *val as i16,
                    _ => {
                        return Err(String::from("CPU_VERSION is not an integer"));
                    }
                };

                let flags_json_option = json_map.get("FLAGS");
                let flags_json = match flags_json_option {
                    Some(val) => val.clone(),
                    None => JSONtype::List(Vec::new()),
                };
                let flags_list = match flags_json {
                    JSONtype::List(list) => list.clone(),
                    _ => {
                        return Err(String::from("FLAGS field not a list"));
                    },
                };
                let mut flags = Vec::<String>::new();
                for flag in flags_list {
                    flags.push(match flag {
                        JSONtype::String(s) => s,
                        _ => {
                            return Err(String::from("Non string type in flags list"));
                        }
                    });
                }

                Ok(Cpu{
                    registers: registers,
                    ram: ram,
                    version: cpu_version,
                    flags: flags,
                })

            }
            _ => Err(String::from("Passed json is not an object"))
        }
    }

    pub fn to_json(self: Cpu) -> JSONtype {
        let register_json = self.registers.to_json();
        let ram_json = self.ram.to_json();
        let version_json = JSONtype::Int(self.version as i64);
        let flags_list = self.flags.iter().map(|s| JSONtype::String(s.clone())).collect();
        let flags_json = JSONtype::List(flags_list);
        let mut json_map = HashMap::<String, JSONtype>::new();
        json_map.insert(String::from("REGISTER_FILE"), register_json);
        json_map.insert(String::from("RAM"), ram_json);
        json_map.insert(String::from("CPU_VERSION"), version_json);
        json_map.insert(String::from("FLAGS"), flags_json);
        JSONtype::Object(json_map)
    }
}