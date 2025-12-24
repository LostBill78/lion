use bitfield_struct::bitfield;

#[derive(Clone)]
pub struct InputBuffer {
    pub buffer: Vec<u8>,
    pub buffer_length: u32,
    pub input_length: u32,
}


#[derive(Debug, Default)]
pub enum DataType {
    U8,
    U16,
    I16,
    U32,
    I32,
    U64,
    #[default]
    I64,
    String,
    Blob,
}

#[bitfield(u8)]
pub struct TypeDefine {
    #[bits(4)]
    base: usize,
    allow_empty: bool,
    is_index: bool,
    is_unique: bool,
    is_present: bool,
}

#[derive(Debug)]
pub struct Column {
    pub column_name: String,
    pub column_data_type: DataType,
    pub column_control: TypeDefine,
}

impl Default for Column {
    fn default() -> Self {
        Self { 
            column_name: Default::default(), 
            column_data_type: Default::default(), 
            column_control: Default::default() }
    }
}

pub struct Table {
    table_name: String,
    columns: Vec<Column>,
}

pub struct Page {
    pub content: [u8; 4096],
}
impl Default for Page {
    fn default() -> Self {
        Self { content: [0; 4096] }
    }
}

pub struct Pages {
    pub pages: Vec<Page>,
    pub num_pages: u8,
}
