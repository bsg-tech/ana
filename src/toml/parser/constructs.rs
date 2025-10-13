use std::{
    fmt,
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub struct Toml {
    pub tables: Vec<Table>,
}

impl Toml {
    pub fn new() -> Self {
        Toml { tables: Vec::new() }
    }

    fn bytes(&self) -> Vec<u8> {
        let mut bv = Vec::new();

        for t in self.tables.iter() {
            bv.extend_from_slice(t.bytes().as_slice());
            bv.push(b'\n');
        }

        bv
    }

    pub fn write_to_file(
        &self,
        path: &str,
        append: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .append(append)
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        // TODO: Is this idiomatic?
        let mut writer = BufWriter::new(file);
        writer.write(self.bytes().as_slice())?;
        writer.flush()?;

        Ok(())
    }
}

impl fmt::Display for Toml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, table) in self.tables.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", table)?;
            } else {
                write!(f, "\n\n{}", table)?;
            }
        }

        write!(f, "")
    }
}

pub struct Table {
    pub header: String,
    pub key_vals: Vec<KeyVal>,
}

impl Table {
    pub fn new(header: String) -> Self {
        Table {
            header,
            key_vals: Vec::new(),
        }
    }

    fn bytes(&self) -> Vec<u8> {
        let mut bv = Vec::new();

        bv.extend_from_slice(self.header.as_bytes());
        bv.push(b'\n');

        for kv in self.key_vals.iter() {
            bv.extend_from_slice(kv.bytes().as_slice());
            bv.push(b'\n');
        }

        bv
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.header)?;
        for kv in self.key_vals.iter() {
            write!(f, "{}\n", kv)?;
        }
        write!(f, "----")
    }
}

pub struct KeyVal {
    pub key: String,
    pub val: Value,
}

impl KeyVal {
    pub fn new(key: String, val: Option<Value>) -> Self {
        if let Some(val) = val {
            KeyVal { key, val }
        } else {
            KeyVal {
                key,
                val: Value::String(String::new()),
            }
        }
    }

    pub fn from_strings(key: &str, val: &str) -> Self {
        KeyVal {
            key: String::from(key),
            val: Value::String(String::from(val)),
        }
    }

    fn bytes(&self) -> Vec<u8> {
        let mut bv = Vec::new();

        bv.extend_from_slice(self.key.as_bytes());
        bv.push(b'=');
        bv.extend_from_slice(self.val.bytes().as_slice());

        return bv;
    }
}

impl fmt::Display for KeyVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key: {}, Val: {}", self.key, self.val)
    }
}

pub enum Value {
    String(String),
    ArrayOfStrings(Vec<String>),
}

impl Value {
    // Is this the right way to do this?
    fn bytes(&self) -> Vec<u8> {
        match &self {
            Value::String(string_field) => {
                let mut bv = Vec::new();
                bv.extend_from_slice(string_field.as_bytes());

                bv
            }
            Value::ArrayOfStrings(arr_field) => {
                let mut bv = Vec::new();
                bv.push(b'[');

                for (i, string_val) in arr_field.iter().enumerate() {
                    bv.extend_from_slice(string_val.as_bytes());

                    if i < arr_field.len() - 1 {
                        bv.push(b',');
                    }
                }

                bv.push(b']');

                bv
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(str_val) => write!(f, "{}", str_val),
            Value::ArrayOfStrings(arr_val) => {
                write!(f, "[")?;
                for (i, item) in arr_val.iter().enumerate() {
                    write!(f, "{}", item)?;
                    if i < arr_val.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}
