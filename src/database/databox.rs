pub trait DataBox {}

enum TypeId {
    BOOL,
    INT,
    FLOAT,
    STRING,
    LONG,
    ByteArray,
}

struct Type {
    size_in_bytes: i32,
    typeid: TypeId,
}

impl Type {
    pub fn new(size_in_bytes: i32, typeid: TypeId) -> Self {
        Type { size_in_bytes, typeid }
    }
}
