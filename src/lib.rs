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

#[repr(C)]
struct sqsh_superblock {
    magic: u32,
    inode_count: u32,
    modification_time: u32,
    block_size: u32,
    fragment_entry_count: u32,
    compression_id: u16,
    block_log: u16,
    flags: u16,
    id_count: u16,
    version_major: u16,
    version_minor: u16,
    root_inode_ref: u64,
    bytes_used: u64,
    id_table_start: u64,
    xattr_id_table_start: u64,
    inode_table_start: u64,
    directory_table_start: u64,
    fragment_table_start: u64,
    export_table_start: u64,
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
