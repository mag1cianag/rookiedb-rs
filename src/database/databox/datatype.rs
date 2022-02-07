use super::typeid::TypeId;

pub struct DataType {
    typeid: TypeId,
    size_in_bytes: i32,
}

impl DataType {
    fn bool_type() -> DataType {
        DataType {
            typeid: TypeId::BOOL,
            size_in_bytes: 1,
        }
    }
    fn int_type() -> DataType {
        DataType {
            typeid: TypeId::INT,
            size_in_bytes: 4,
        }
    }
    fn float_type() -> DataType {
        DataType {
            typeid: TypeId::FLOAT,
            size_in_bytes: 4,
        }
    }
    fn string_type(n: i32) -> DataType {
        DataType {
            typeid: TypeId::STRING,
            size_in_bytes: n,
        }
    }
    fn long_type() -> DataType {
        DataType {
            typeid: TypeId::LONG,
            size_in_bytes: 6,
        }
    }
    fn byte_array_type(n: i32) -> DataType {
        DataType {
            typeid: TypeId::ByteArray,
            size_in_bytes: n,
        }
    }
}
