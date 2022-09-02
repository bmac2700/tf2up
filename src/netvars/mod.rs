use std::{collections::HashMap, ffi::CStr};

pub fn netvar_hash(string: &str) -> usize {
    let mut output: usize = 0;

    for (i, c) in string.chars().enumerate() {
        output += c as usize * i;
    }

    output
}

fn recursive_dump(
    base_class: String,
    recv_table: *const u8,
    offset: i32,
    hashmap: &mut HashMap<usize, usize>,
) {
    let props_count = unsafe { *((recv_table as usize + 0x4) as *const i32) };

    for i in 0..props_count as usize {
        let prop: *const u8 = unsafe { *(recv_table as *const usize) + 0x3C * i } as *const u8;

        if prop.is_null() || unsafe { (*prop).is_ascii_digit() } {
            continue;
        }

        let prop_name = unsafe { CStr::from_ptr((*(prop as *const usize)) as *const i8) };
        let prop_name = prop_name.to_str().unwrap().to_owned();

        if prop_name.eq("baseclass") {
            continue;
        }

        let prop_recv_type = unsafe { *((prop as usize + 0x4) as *const i32) };
        let prop_data_table = unsafe { *((prop as usize + 0x28) as *const usize) as *const u8 };
        let prop_offset = unsafe { *((prop as usize + 0x2C) as *const i32) };

        if prop_recv_type == 6 {
            if unsafe { *((prop_data_table as usize + 0xC) as *const u8) } == 'D' as u8 {
                recursive_dump(base_class.clone(), prop_data_table, prop_offset, hashmap)
            }
        }

        let name = format!("{}->{}", base_class, prop_name);
        let value_offset = offset as usize + prop_offset as usize;

        /*println!(
            "{} / {} @ 0x{:x?}",
            name,
            netvar_hash(&name),
            value_offset
        );*/

        hashmap.insert(netvar_hash(&name), value_offset);
    }
}

pub fn setup_netvars(client_classes: *const u8) -> HashMap<usize, usize> {
    let mut client_class = client_classes;

    let mut hashmap: HashMap<usize, usize> = HashMap::new();

    while !client_class.is_null() {
        let recv_table = unsafe { *((client_class as usize + 0xC) as *const usize) } as *const u8;
        if !recv_table.is_null() {
            let c_str = unsafe {
                CStr::from_ptr((*((client_class as usize + 0x8) as *const usize)) as *const i8)
            };
            let base_class = c_str.to_str().unwrap().to_owned();

            recursive_dump(base_class, recv_table, 0, &mut hashmap);
        }

        unsafe {
            client_class = (*((client_class as usize + 0x10) as *const usize)) as *const u8;
        }
    }

    return hashmap;
}
