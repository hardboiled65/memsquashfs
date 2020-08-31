#ifndef _MEMSQUASHFS_SQUASHFS_H
#define _MEMSQUASHFS_SQUASHFS_H

#include <stdint.h>

/*==================*/
/* Superblock flags */
/*==================*/
#define SQSH_UNCOMPRESSED_INODES 0x0001
#define SQSH_UNCOMPRESSED_DATA 0x0002
#define SQSH_CHECK 0x0004
#define SQSH_UNCOMPRESSED_FRAGMENTS 0x0008
#define SQSH_NO_FRAGMENTS 0x0010
#define SQSH_ALWAYS_FRAGMENTS 0x0020
#define SQSH_DUPLICATES 0x0040
#define SQSH_EXPORTABLE 0x0080
#define SQSH_UNCOMPRESSED_XATTRS 0x0100
#define SQSH_NO_XATTRS 0x0200
#define SQSH_COMPRESSOR_OPTIONS 0x0400
#define SQSH_UNCOMPRESSED_IDS 0x0800


typedef struct sqsh_superblock {
    uint32_t magic;
    uint32_t inode_count;
    uint32_t modification_time;
    uint32_t block_size;
    uint32_t fragment_entry_count;
    uint16_t compression_id;
    uint16_t block_log;
    uint16_t flags;
    uint16_t id_count;
    uint16_t version_major;
    uint16_t version_minor;
    uint64_t root_inode_ref;
    uint64_t bytes_used;
    uint64_t id_table_start;
    uint64_t xattr_id_table_start;
    uint64_t inode_table_start;
    uint64_t directory_table_start;
    uint64_t fragment_table_start;
    uint64_t export_table_start;
} sqsh_superblock;


typedef struct sqsh_squashfs {
    sqsh_superblock superblock;
} sqsh_squashfs;


sqsh_superblock sqsh_superblock_read(const uint8_t *data);

#endif /* _MEMSQUASHFS_SQUASHFS_H */
