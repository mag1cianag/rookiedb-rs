use crate::database::databox::DataBox;
use crate::database::table::RecordId;
use std::iter::Iterator;
trait BPlusNode {
    fn get(&self,key: Box<dyn DataBox>) -> LeafNode;
    fn get_leftmost_leaf(&self) -> LeafNode;
    fn put(&self,key: Box<dyn DataBox>, rid: RecordId) -> Option<(Box<dyn DataBox>, i64)>;
    fn bulk_load(&self,data: Box<dyn Iterator<Item=i32>>, fill_factor: f64) -> Option<(Box<dyn DataBox>, i64)>;
}


struct BPlusTree {
    root: Box<dyn BPlusNode>,

}

impl BPlusTree {
    pub fn get(&self, key : Box<dyn DataBox>) -> Option<RecordId> {
        todo!()
    }
}


struct LeafNode {}

impl BPlusNode for LeafNode {
    fn get(&self, key: Box<dyn DataBox>) -> LeafNode {
        todo!()
    }

    fn get_leftmost_leaf(&self) -> LeafNode {
        todo!()
    }

    fn put(&self, key: Box<dyn DataBox>, rid: RecordId) -> Option<(Box<dyn DataBox>, i64)> {
        todo!()
    }

    fn bulk_load(&self, data: Box<dyn Iterator<Item=i32>>, fill_factor: f64) -> Option<(Box<dyn DataBox>, i64)> {
        todo!()
    }
}

struct InnerNode{}