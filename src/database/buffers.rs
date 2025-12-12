use bitfield_struct::bitfield;

#[derive(Clone)]
pub struct InputBuffer {
    pub buffer: Vec<u8>,
    pub buffer_length: u32,
    pub input_length: u32,
}


pub enum DataType {
    U8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    String,
    Blob,
}

#[bitfield(u8)]
struct TypeDefine {
    #[bits(4)]
    base: usize,
    allow_empty: bool,
    is_index: bool,
    is_unique: bool,
    is_present: bool,
}
pub struct Column {
    column_name: String,
    column_data_type: DataType,
    column_control: TypeDefine,
}

pub struct Table {
    table_name: String,
    columns: Vec<Column>,
}
