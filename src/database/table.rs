use crate::database::databox::DataBox;

pub struct Record {
    values: Vec<Box<dyn DataBox>>,
}

impl Record {
    pub fn get_value(self, i: usize) -> Box<dyn DataBox> {
        todo!()
    }
}

pub struct RecordId {
    page_num: i64,
    entry_num: i16,
}

impl RecordId {
    fn get_size_in_bytes() -> i32 {
        64 + 16
    }
}