use json_rw::JSONtype;
use std::collections::HashMap;

pub struct Registers {
    pc: u16, sp: u16,
    gp: [i16; 16],
}

pub struct RAM {
    ram: [i16; 65536],
}

fn i16_to_hex(value: i16) -> String {
    let adjusted_val = ((value as i32) - (i16::MIN as i32)) as u16;
    format!("{:#06X}", adjusted_val)
}

fn u16_to_hex(value: u16) -> String {
    format!("{:#06X}", value)
}

fn i16_to_hex_compressed(value: i16, mult: usize) -> String {
    format!("{:#06X}*{}", value, mult)
}

fn hex_to_i16(value: &str) -> Option<i16> {
    if value.len() != 6 {
        return None;
    };
    if !value.starts_with("0x") {
        return None;
    }
    let without_prefix = value.trim_start_matches("0x");
    let z = i32::from_str_radix(without_prefix, 16);
    let res: i16;
    if let Ok(v) = z {
        res = (v+(i16::MIN as i32)) as i16;
    } else {
        return None;
    }
    Some(res)
}

fn hex_to_u16(value: &str) -> Option<u16> {
    if value.len() != 6 {
        return None;
    };
    if !value.starts_with("0x") {
        return None;
    }
    let without_prefix = value.trim_start_matches("0x");
    let z = u16::from_str_radix(without_prefix, 16);
    let res: u16;
    if let Ok(v) = z {
        res = v;
    } else {
        return None;
    }
    Some(res)
}

fn hex_to_i16_compressed(value: &str) -> Option<(i16, usize)> {
    if !value.contains('*') {
        return None;
    }
    if value.split('*').count() != 2 {
        return None;
    }
    let mut split = value.split('*');
    let val = hex_to_i16(split.next().unwrap());
    if let None = val {
        return None;
    }
    let val = val.unwrap();
    let mult_res= split.next().unwrap().parse::<usize>();
    let mult: usize;
    if let Ok(mult_val) = mult_res {
        mult = mult_val;
    } else {
        return None
    }
    Some((val, mult))
}



impl Registers {
    pub fn default() -> Registers {
        Registers {
            pc: 0,
            sp: u16::MAX,
            gp: [0i16; 16],
        }
    }

    pub fn from_json(json_map: &HashMap<String, JSONtype>) -> Result<Registers, String> {
        // If the program file contains no REGISTER_FILE key, just return the default
        if !json_map.contains_key("REGISTER_FILE") {
            return Ok(Self::default());
        }

        // Get the value from REGISTER_FILE and check that is is a JSON::Object
        let reg_file_json = json_map.get("REGISTER_FILE").unwrap();
        let reg_file_map = match reg_file_json {
            JSONtype::Object(reg_file_map) => reg_file_map,
            _ => {
                return Err("REGISTER_FILE field is not an object".to_string());
            },
        };

        // Get the PC register value with error checking
        if !reg_file_map.contains_key("PC") {
            return Err("REGISTER_FILE contains no PC entry".to_string());
        };
        let pc_json = reg_file_map
            .get("PC")
            .unwrap();
        let pc: u16;
        if let JSONtype::String(pc_str) = pc_json {
            if let Some(pc_val) = hex_to_u16(pc_str) {
                pc = pc_val;
            } else {
                return Err("Incorrect register format for PC".to_string());
            }
        } else {
            return Err("PC entry is not a string".to_string());
        }

        // Get the SP register value with error checking
        if !reg_file_map.contains_key("SP") {
            return Err("REGISTER_FILE contains no SP entry".to_string());
        };
        let sp_json = reg_file_map
            .get("SP")
            .unwrap();
        let sp: u16;
        if let JSONtype::String(sp_str) = sp_json {
            if let Some(sp_val) = hex_to_u16(sp_str) {
                sp = sp_val;
            } else {
                return Err("Incorrect register format for PC".to_string());
            }
        } else {
            return Err("SP entry is not a string".to_string());
        }

        // Get the general purpose registers with error checking
        if !reg_file_map.contains_key("REGISTERS") {
            return Err("REGISTER_FILE contains no REGISTERS entry".to_string());
        };
        let registers = reg_file_map
            .get("REGISTERS")
            .unwrap();
        let register_list: &Vec<JSONtype>;
        if let JSONtype::List(list) = registers {
            register_list = list;
        } else {
            return Err("REGISTERS field not a list".to_string());
        }
        if register_list.len() != 16 {
            return Err("REGISTERS list not correct length".to_string());
        }
        let mut list = [0i16; 16];
        // Will only execute if register_list.len()==16, thus can index list aswell
        for (i, register_item) in register_list.iter().enumerate() {
            if let JSONtype::String(reg_val) = register_item {
                if let Some(v) = hex_to_i16(reg_val) {
                    list[i] = v;
                } else {
                    return Err("REGISTERS field contains incorrect format".to_string());
                }
            } else {
                return Err("REGISTERS list contains non string value".to_string());
            }
        }
        Ok(Registers { pc: pc, sp: sp, gp: list })
    }

    pub fn to_json(self: Registers) -> JSONtype {
        let gp_json = JSONtype::List(Vec::from(self.gp.map(|r| JSONtype::String(i16_to_hex(r)))));
        let mut map_json = HashMap::<String, JSONtype>::new();
        map_json.insert(String::from("REGISTERS"), gp_json);
        map_json.insert(String::from("PC"), JSONtype::String(u16_to_hex(self.pc)));
        map_json.insert(String::from("SP"), JSONtype::String(u16_to_hex(self.sp)));
        JSONtype::Object(map_json)
    }
}

impl RAM {
    pub fn default() -> RAM {
        RAM {
            ram: [0; u16::MAX as usize+1],
        }
    }

    pub fn from_json(_json_map: &HashMap<String, JSONtype>) -> Result<RAM, String> {
        Ok(Self::default())
    }
    pub fn to_json(self: RAM) -> JSONtype {
        // TODO: add compression for smaller files
        let mut ram_list = Vec::<JSONtype>::new();
        let ram_len = self.ram.len();
        let mut i = 0;
        while i < ram_len {
            let val = self.ram[i];
            let mut mult: usize = 1;
            while (i+mult < ram_len) && (val == self.ram[i+mult]) {
                mult += 1;
            };
            if mult > 1 {
                i += mult;
                ram_list.push(JSONtype::String(i16_to_hex_compressed(val, mult)));
            } else {
                i += 1;
                ram_list.push(JSONtype::String(i16_to_hex(val)));
            }
        };
        JSONtype::List(ram_list)
    }
}