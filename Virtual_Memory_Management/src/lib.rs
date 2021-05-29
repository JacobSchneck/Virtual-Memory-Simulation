use std::fs::File;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::SeekFrom;

// GLOBALS
pub const NUM_PAGES: u32 = 256;
pub const NUM_FRAMES: u32 = 128;
pub const PAGE_SIZE: u32 = 256;

pub struct PageTableInfo {
    page_table: [u32; NUM_PAGES as usize],
    access_time: [u32; NUM_FRAMES as usize],
    free_frame: [u32; NUM_FRAMES as usize],
}

pub fn decode_address(address: u32, page_number: &mut u32, page_offset: &mut u32) {
    if address < 0 || address > PAGE_SIZE*NUM_PAGES {
        panic!("Error: Virtual memory address out of bounds");
    }

    *page_number = (address >> 8) & (PAGE_SIZE - 1);
    *page_offset = address & (PAGE_SIZE - 1);
}

pub fn read_from_backing_store(f: &mut File, buffer: &mut [u8], page_number: u32) {
    f.seek(SeekFrom::Start( (PAGE_SIZE * page_number).into() ));
    f. read(buffer);
}

#[cfg(test)]
mod tests {
    use super::*;

    /********************************** Test decode_address(..) *********************************/
    #[test]
    fn test_decode_address() {
        let mut page_number: u32 = 0;
        let mut page_offset: u32 = 0;

        // Test One 
        decode_address(0, &mut page_number, &mut page_offset);
        assert!(page_number == 0 && page_offset == 0);

        // Test Two
        decode_address(1, &mut page_number, &mut page_offset);
        assert!(page_number == 0 && page_offset == 1);
        
        // Test Three 
        decode_address(256, &mut page_number, &mut page_offset);
        assert!(page_number == 1 && page_offset == 0);

        // Test Four 
        decode_address(256, &mut page_number, &mut page_offset);
        assert!(page_number == 1 && page_offset == 0);

        // Test Five
        decode_address(32768, &mut page_number, &mut page_offset);
        assert!(page_number == 128 && page_offset == 0);

        // Test Six
        decode_address(32769, &mut page_number, &mut page_offset);
        assert!(page_number == 128 && page_offset == 1);

        // Test Seven
        decode_address(128, &mut page_number, &mut page_offset);
        assert!(page_number == 0 && page_offset == 128);

        // Test Eight
        decode_address(65534, &mut page_number, &mut page_offset);
        assert!(page_number == 255 && page_offset == 254);

        // Test Nine
        decode_address(33153, &mut page_number, &mut page_offset);
        assert!(page_number == 129 && page_offset == 129);

        // Test Ten
        decode_address(16916, &mut page_number, &mut page_offset);
        assert!(page_number == 66 && page_offset == 20);

        // Test Eleven
        decode_address(62493, &mut page_number, &mut page_offset);
        assert!(page_number == 244 && page_offset == 29);
    }

    #[test]
    #[should_panic(expected = "Error: Virtual memory address out of bounds")]
    fn test_decode_address_panic() {
        let mut page_number: u32 = 0;
        let mut page_offset: u32 = 0;
        decode_address(PAGE_SIZE*NUM_PAGES + 1, &mut page_number, &mut page_offset); 
    }

    /********************************** Test read_from_backing_store(..) ************************/
    #[test]
    fn test_read_from_backing_store() {
        let filename = String::from("BACKING_STORE.dat");
        let mut buffer: [u8; PAGE_SIZE as usize] = [0; PAGE_SIZE as usize];
        let mut f = File::open(filename).unwrap();

        read_from_backing_store(&mut f, &mut buffer, 0);
        assert!(buffer[0] == 0 && buffer[255] == 63 && buffer[171] == 42);
    }
}
    