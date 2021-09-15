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

impl std::fmt::Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Int8 => f.write_str("Int8"),
            Types::Int16 => f.write_str("Int16"),
            Types::Int32 => f.write_str("Int32"),
            Types::Int64 => f.write_str("Int64"),
            Types::Int128 => f.write_str("Int128"),
            Types::Int256 => f.write_str("Int256"),
            Types::UInt8 => f.write_str("UInt8"),

            Types::UInt16 => f.write_str("UInt16"),
            Types::UInt32 => f.write_str("UInt32"),
            Types::UInt64 => f.write_str("UInt64"),
            Types::UInt128 => f.write_str("UInt128"),
            Types::UInt256 => f.write_str("UInt256"),
            Types::Float32 => f.write_str("Float32"),
            Types::Float64 => f.write_str("Float64"),
            Types::Decimal32 => f.write_str("Decimal32"),
            Types::Decimal64 => f.write_str("Decimal64"),
            Types::Decimal128 => f.write_str("Decimal128"),
            Types::Decimal256 => f.write_str("Decimal256"),
            Types::Varchar(length) => f.write_str(&format!("VARCHAR({})", &length.to_string())),
            Types::UUID => f.write_str("UUID"),
            Types::Date => f.write_str("Date"),
            Types::Date32 => f.write_str("Date32"),

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
    primary: bool,
    nullable: bool,
    unique: bool
}

impl Column {
    pub fn new(name: String, r#type: Types) -> Column {
        Column {
            name,
            r#type,
            primary: false,
            unique: false,
            nullable: true
        }
    }

    pub fn primary(self) -> Column {
        Self {
            primary: true,
            ..self
        }
    }

    pub fn unique(self) -> Column {
        Self {
            unique: true,
            ..self
        }
    }

    pub fn nullable(self) -> Column {
        Self {
            nullable: false,
            ..self
        }
    }
}