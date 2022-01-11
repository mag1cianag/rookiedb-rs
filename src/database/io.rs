use std::collections::HashMap;
use std::hash::Hash;
use std::{fs, io};
use std::fs::create_dir;

const PAGE_SIZE: i32 = 4096;
// size of a page in bytes
const INVALID_PAGE_NUM: i64 = -1;
// a page number that is always invalid
const FACTOR: i64 = 10_000_000_000;

// translation factor
trait DiskSpaceManager: Drop
{
    /// Allocates a new partition.
    fn alloc_part() -> i32;

    /// Allocates a new partition with a specific partition number.
    fn alloc_part_by_part_num(part_num: i32) -> i32;

    /// Gets the virtual page number given partition/data page number
    fn get_virtual_page_num(part_num: i32, page_num: i32) -> i64 {
        part_num as i64 * FACTOR + page_num as i64
    }

    /// Gets partition number from virtual page number
    fn get_part_num(page: i64) -> i32 {
        (page / FACTOR) as i32
    }

    /// Gets data page number from virtual page number
    fn get_page_num(page: i64) -> i32 {
        (page % FACTOR) as i32
    }
}

struct OriDiskSpaceManager {
    db_dir: String,
    part_info: HashMap<i32, PartitionHandle>,
}


impl DiskSpaceManager for OriDiskSpaceManager {
    fn alloc_part() -> i32 {
        todo!()
    }

    fn alloc_part_by_part_num(part_num: i32) -> i32 {
        todo!()
    }

    fn get_virtual_page_num(part_num: i32, page_num: i32) -> i64 {
        todo!()
    }

    fn get_part_num(page: i64) -> i32 {
        todo!()
    }

    fn get_page_num(page: i64) -> i32 {
        todo!()
    }
}

impl Drop for OriDiskSpaceManager {
    fn drop(&mut self) {
        todo!()
    }
}

impl OriDiskSpaceManager {
    pub fn new(db_dir: String) -> Result<Self, io::Error> {
        if let Ok(d)=fs::read_dir(&db_dir) {
           let mut max_file_num = -1;
        }else {
            match create_dir(&db_dir) {
                Ok(_) => {}
                Err(e) => {
                    return Err(
                        io::Error::new(
                            e.kind(),
                            "could not initialize disk space manager - directory is a file"
                        )
                    );
                }
            }
        }


        Ok(OriDiskSpaceManager { db_dir, part_info: HashMap::new() })
    }
}

struct PartitionHandle {}
