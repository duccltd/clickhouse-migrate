use std::fmt::Formatter;

pub struct Table {
    name: String,
    columns: Vec<Column>,
    exists: bool
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            name,
            columns: Vec::new(),
            exists: false
        }
    }

    pub fn new_exists(name: String, exists: bool) -> Table {
        Table {
            name,
            columns: Vec::new(),
            exists,
        }
    }

    pub fn column(mut self, name: &str, r#type: Types) -> &mut Column {
        let mut column = Column::new(
            name: name.to_string(),
            r#type
        );
        self.columns.push(column);

        &mut column
    }

    pub fn as_str(&self) -> &'static str {
        unimplemented!("table to raw form")
    }
}

pub enum Types {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int256,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    UInt256,
    Float32,
    Float64,
    Decimal32,
    Decimal64,
    Decimal128,
    Decimal256,
    Varchar(u16),
    UUID,
    Date,
    Date32,

    // TODO: DateTime types
    DateTime(u8),
    DateTime64(u8),

    // TODO: Enum type
    Enum,
}

impl Types {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Types::Int8 => "Int8",
            Types::Int16 => "Int16",
            Types::Int32 => "Int32",
            Types::Int64 => "Int64",
            Types::Int128 => "Int128",
            Types::Int256 => "Int256",
            Types::UInt8 => "UInt8",

            Types::UInt16 => "UInt16",
            Types::UInt32 => "UInt32",
            Types::UInt64 => "UInt64",
            Types::UInt128 => "UInt128",
            Types::UInt256 => "UInt256",
            Types::Float32 => "Float32",
            Types::Float64 => "Float64",
            Types::Decimal32 => "Decimal32",
            Types::Decimal64 => "Decimal64",
            Types::Decimal128 => "Decimal128",
            Types::Decimal256 => "Decimal256",
            Types::Varchar(length) => &format!("VARCHAR({})", &length.to_string()),
            Types::UUID => "UUID",
            Types::Date => "Date",
            Types::Date32 => "Date32",

            // TODO: DateTime types
            Types::DateTime(region) => format!("DateTime({})", region),
            Types::DateTime64(region) => format!("DateTime64({})", region),

            // TODO: Enum type
            Types::Enum => "Enum"
        }
    }
}

pub struct Column {
    name: String,
    r#type: Types,
    nullable: bool,
}

impl Column {
    pub fn new(name: String, r#type: Types) -> Column {
        Column {
            name,
            r#type,
            nullable: true
        }
    }

    pub fn nullable(self) -> Column {
        Self {
            nullable: false,
            ..self
        }
    }

    pub fn as_str(&self) -> &'static str {
        let entry: &str = &format!("{} {}", self.name, self.r#type.as_str());
        if !self.nullable {
            entry = entry + " NOT NULL "
        }

        &entry[0..entry.len() - 1]
    }
}