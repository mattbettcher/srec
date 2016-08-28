// Reader for Motorola's S-Record file format.

use std::io;
use std::fs::File;
use std::io::prelude::*;

// reads an s-record and returns the base address and a vector of words
pub fn read_srec(filename: &'static str) -> Result<(u32, Vec<(u32, Vec<u8>)>) ,io::Error> {
    let mut f = try!(File::open(filename));
    let mut b = Vec::new();
    let mut result: Vec<(u32, Vec<u8>)> = Vec::new();

    try!(f.read_to_end(&mut b));

    let mut i = 0;
    let mut execute_address = 0;

    loop {
        if i >= b.len() { break; }
        let rec_type = read_chars(&b, &mut i, 2, true);
        let count = u16::from_str_radix(read_chars(&b, &mut i, 2, true).as_str(), 16).unwrap() * 2;
        let base_address_u16 = u32::from_str_radix(read_chars(&b, &mut i, 4, false).as_str(), 16).unwrap();
        let base_address_u24 = u32::from_str_radix(read_chars(&b, &mut i, 6, false).as_str(), 16).unwrap();
        let base_address_u32 = u32::from_str_radix(read_chars(&b, &mut i, 8, false).as_str(), 16).unwrap();

        match rec_type.as_str() {
            "S0" => {
                // skip this record for now
               i += (count + 2) as usize;
               println!("S0 matched : new index {}", i);
            },
            "S1" => {
                i += 4;
                println!("S1 matched : data starts at {} for bytes : {}", i, count - 8);
                let mem = read_bytes(&b, &mut i, (count - 8) as usize);
                result.push((base_address_u16 as u32, mem.clone()));
                i += 4;
                println!("current index : {}, mem.len : {}", i, mem.len());
            },
            "S2" => {
                i += 6;
                println!("S2 matched : data starts at {} for bytes : {}", i, count - 10);
                let mem = read_bytes(&b, &mut i, (count - 10) as usize);
                result.push((base_address_u24 as u32, mem.clone()));
                i += 4;
                println!("current index : {}, mem.len : {}", i, mem.len());
            },
            "S3" => {
                i += 8;
                println!("S3 matched : data starts at {} for bytes : {}", i, count - 12);
                let mem = read_bytes(&b, &mut i, (count - 12) as usize);
                result.push((base_address_u32 as u32, mem.clone()));
                i += 4;
                println!("current index : {}, mem.len : {}", i, mem.len());
            },
            "S5" => {
                // this should include the total records in the address field
                // we just skip it
                i += (count + 2) as usize;
            },
            "S7" => {
                execute_address = base_address_u32;
                i += (count + 2) as usize;
            },
            "S8" => {
                execute_address = base_address_u24;
                i += (count + 2) as usize;
            },
            "S9" => {
                execute_address = base_address_u16;
                i += (count + 2) as usize;
            },
            e => panic!("{:?} - doesn't have a matching pattern!", e),
        }
    }

    Ok((execute_address, result))
}

// use slice syntax
fn read_chars(buffer: &[u8], start: &mut usize, len: usize, inc_index: bool) -> String {
    let s = String::from_utf8_lossy(&buffer[*start..*start+len]).into_owned();
    if inc_index { *start += len; }
    s
}

fn read_bytes(buffer: &[u8], i: &mut usize, len: usize) -> Vec<u8> {
    let mut mem = Vec::new();
    let start = *i;
    loop {
        if *i > start + len as usize { break; }
        let v = u8::from_str_radix(read_chars(&buffer, &mut *i, 2, true).as_str(), 16).unwrap();
        mem.push(v as u8);                   
    }
    mem
}

#[cfg(not(test))]
fn main() {
    let v = read_srec("tutorial4.S68").unwrap();

    println!("0x{:04x} : excute address", v.0);
    for i in v.1.iter() {
        println!("{:x} : base address", i.0);
        for j in i.1.iter() {
            println!("0x{:02x}", j);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
