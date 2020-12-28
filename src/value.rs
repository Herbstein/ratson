use std::collections::HashMap;

use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize, Serializer,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Uint(u64),
    Float(f64),
    String(Vec<u8>),
    Object(HashMap<Vec<u8>, Value>),
    Array(Vec<Value>),
    Bool(bool),
    Nil,
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Int(i) => serializer.serialize_i64(*i),
            Value::Uint(u) => serializer.serialize_u64(*u),
            Value::Float(f) => serializer.serialize_f64(*f),
            Value::String(s) => {
                let s = String::from_utf8_lossy(s);
                serializer.serialize_str(&s)
            }
            Value::Object(o) => {
                let mut map = serializer.serialize_map(Some(o.len()))?;
                for (k, v) in o {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            Value::Array(a) => {
                let mut seq = serializer.serialize_seq(Some(a.len()))?;
                for e in a {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Nil => serializer.serialize_unit(),
        }
    }
}
