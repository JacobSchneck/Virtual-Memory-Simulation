use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use Virtual_Memory_Management::*;

fn main() {
    let mut f_b: File = File::open("BACKING_STORE.dat").unwrap();
    let mut info: PageTableInfo = PageTableInfo::new();

    let mut access_time = 0;
    let mut page_number: u32 = 0;
    let mut page_offset: u32 = 0;
    let mut page_fault = false;
    let mut buffer: [u8; PAGE_SIZE as usize] = [0; PAGE_SIZE as usize];
    // let mut physical_memory;  
    let mut physical_memory = vec![&[0; PAGE_SIZE as usize]; (PAGE_SIZE*NUM_FRAMES) as usize];
    
    if let Ok(lines) = read_lines("./addresses.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let address: u32 = ip.parse::<u32>().unwrap();
                decode_address(address, &mut page_number, &mut page_offset);
                let frame_number = get_frame_number(&mut info, page_number, access_time, &mut page_fault);
                if page_fault {
                    read_from_backing_store(&mut f_b, &mut buffer, page_number);
                    // physical_memory[(frame_number*(PAGE_SIZE as i32)) as usize];
                    println!("* Virtual Address: {} [{}, {}] Physical Address: {} [{}, {}]",
                        address,
                        page_number,
                        page_offset,
                        (frame_number*(PAGE_SIZE as i32) + page_offset as i32),
                        frame_number,
                        page_offset,
                        // physical_memory[(frame_number*(PAGE_SIZE as i32) + page_offset as i32) as usize],
                    );
                } else {
                    println!("Virtual Address: {} [{}, {}] Physical Address: {} [{}, {}]",
                        address,
                        page_number,
                        page_offset,
                        (frame_number*(PAGE_SIZE as i32) + page_offset as i32),
                        frame_number,
                        page_offset,
                        // physical_memory[(frame_number*(PAGE_SIZE as i32) + page_offset as i32) as usize],
                    );
                }
                access_time += 1;
            }
        }
    }
    println!("{}", access_time);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}