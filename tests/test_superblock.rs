use memsquashfs::sqsh_superblock;
use memsquashfs::sqsh_superblock_read;

use std::fs::File;
use std::io::Read;

#[test]
fn test_superblock() {
    let mut f = match File::open("tests/data") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error open data file. {:?}", e);
            return;
        }
    };

    let mut data: Vec<u8> = vec![];
    match f.read_to_end(&mut data) {
        Ok(_) => {}
        Err(e) => { eprintln!("read error. {:?}", e) }
    }
    let data = data.as_ptr();
    println!("data: {:?}", data);

    let superblock = sqsh_superblock_read(data);
    println!("magic: 0x{:8X}", superblock.magic);
    println!("compression_id: {}", superblock.compression_id);
}