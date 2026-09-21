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
    format!("{:#06x}", value)
}

fn i16_to_hex_compressed(value: i16, mult: usize) -> String {
    format!("{:#06x}x{}", value, mult)
}

impl Registers {
    pub fn default() -> Registers {
        Registers {
            pc: 0,
            sp: u16::MAX,
            gp: [0i16; 16],
        }
    }

    pub fn from_json(_json_map: &HashMap<String, JSONtype>) -> Result<Registers, String> {
        Ok(Self::default())
    }

    pub fn to_json(self: Registers) -> JSONtype {
        let gp_json = JSONtype::List(Vec::from(self.gp.map(|r| JSONtype::String(i16_to_hex(r)))));
        let mut map_json = HashMap::<String, JSONtype>::new();
        map_json.insert(String::from("REGISTERS"), gp_json);
        map_json.insert(String::from("PC"), JSONtype::String(format!("{:#06x}", self.pc)));
        map_json.insert(String::from("SP"), JSONtype::String(format!("{:#06x}", self.sp)));
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