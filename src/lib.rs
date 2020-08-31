use byteorder::{ByteOrder, LittleEndian};

// Superblock flags.
const UNCOMPRESSED_INODES: u16 = 0x0001;
const UNCOMPRESSED_DATA: u16 = 0x0002;
const CHECK: u16 = 0x0004;
const UNCOMPRESSED_FRAGMENTS: u16 = 0x0008;
const NO_FRAGMENTS: u16 = 0x0010;
const ALWAYS_FRAGMENTS: u16 = 0x0020;
const DUPLICATES: u16 = 0x0040;
const EXPORTABLE: u16 = 0x0080;
const UNCOMPRESSED_XATTRS: u16 = 0x0100;
const NO_XATTRS: u16 = 0x0200;
const COMPRESSOR_OPTIONS: u16 = 0x0400;
const UNCOMPRESSED_IDS: u16 = 0x0800;

// Compression ids.
const GZIP: u16 = 1;
const LZMA: u16 = 2;
const LZO: u16 = 3;
const XZ: u16 = 4;
const LZ4: u16 = 5;
const ZSTD: u16 = 6;

const SUPERBLOCK_SIZE: usize = 96;

#[repr(C)]
pub struct sqsh_superblock {
    pub magic: u32,                 // 4
    pub inode_count: u32,           // 8
    pub modification_time: u32,     // 12
    pub block_size: u32,            // 16
    pub fragment_entry_count: u32,  // 20
    pub compression_id: u16,        // 22
    pub block_log: u16,             // 24
    pub flags: u16,                 // 26
    pub id_count: u16,              // 28
    pub version_major: u16,         // 30
    pub version_minor: u16,         // 32
    pub root_inode_ref: u64,        // 40
    pub bytes_used: u64,            // 48
    pub id_table_start: u64,        // 56
    pub xattr_id_table_start: u64,  // 64
    pub inode_table_start: u64,     // 72
    pub directory_table_start: u64, // 80
    pub fragment_table_start: u64,  // 88
    pub export_table_start: u64,    // 96
}

#[repr(C)]
pub struct sqsh_squashfs {
    superblock: sqsh_superblock,
}

#[no_mangle]
pub extern "C" fn sqsh_superblock_read(data: *const u8) -> sqsh_superblock {
    let mut offset = 0;
    let data = unsafe {
        std::slice::from_raw_parts(data, SUPERBLOCK_SIZE)
    };

    let magic = LittleEndian::read_u32(&data[..]);
    offset += 4;
    let inode_count = LittleEndian::read_u32(&data[offset..]);
    offset += 4;
    let modification_time = LittleEndian::read_u32(&data[offset..]);
    offset += 4;
    let block_size = LittleEndian::read_u32(&data[offset..]);
    offset += 4;
    let fragment_entry_count = LittleEndian::read_u32(&data[offset..]);
    offset += 4;
    let compression_id = LittleEndian::read_u16(&data[offset..]);
    offset += 2;
    let block_log = LittleEndian::read_u16(&data[offset..]);
    offset += 2;

    sqsh_superblock {
        magic: magic,
        inode_count: inode_count,
        modification_time: modification_time,
        block_size: block_size,
        fragment_entry_count: fragment_entry_count,
        compression_id: compression_id,
        block_log: block_log,
        flags: 0,
        id_count: 0,
        version_major: 0,
        version_minor: 0,
        root_inode_ref: 0,
        bytes_used: 0,
        id_table_start: 0,
        xattr_id_table_start: 0,
        inode_table_start: 0,
        directory_table_start: 0,
        fragment_table_start: 0,
        export_table_start: 0,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_byteorder() {
        use byteorder::{ByteOrder, LittleEndian};

        let data: Vec<u8> = vec![0x34, 0x56, 0x78, 0x90];
        let read = LittleEndian::read_u32(&data);
        println!("{:8X}", read);
        println!("{:8X}", LittleEndian::read_u32(&data));
    }
}
