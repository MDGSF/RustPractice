use std::collections::HashMap;
pub use value::*;

pub trait FromValue<T>: Default {
    fn fromValue(self) -> T;
}

pub trait ToValue<T>: Default {
    fn toValue(self) -> Value;
}

impl FromValue<bool> for Value {
    fn fromValue(self) -> bool {
        match self {
            Value::Bool(n) => n,
            _ => panic!("invalid value for bool"),
        }
    }
}

impl FromValue<String> for Value {
    fn fromValue(self) -> String {
        match self {
            Value::String(s) => s,
            _ => panic!("invalid value for String"),
        }
    }
}

impl FromValue<Vec<Value>> for Value {
    fn fromValue(self) -> Vec<Value> {
        match self {
            Value::Array(a) => a,
            _ => panic!("invalid value for Array"),
        }
    }
}

impl FromValue<HashMap<String, Value>> for Value {
    fn fromValue(self) -> HashMap<String, Value> {
        match self {
            Value::Object(hm) => hm,
            _ => panic!("invalid value for bool"),
        }
    }
}

impl FromValue<u8> for Value {
    fn fromValue(self) -> u8 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<u8>().unwrap();
                num
            }
            _ => panic!("invalid value for u8"),
        }
    }
}

impl FromValue<i8> for Value {
    fn fromValue(self) -> i8 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<i8>().unwrap();
                num
            }
            _ => panic!("invalid value for i8"),
        }
    }
}

impl FromValue<u16> for Value {
    fn fromValue(self) -> u16 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<u16>().unwrap();
                num
            }
            _ => panic!("invalid value for u16"),
        }
    }
}

impl FromValue<i16> for Value {
    fn fromValue(self) -> i16 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<i16>().unwrap();
                num
            }
            _ => panic!("invalid value for i16"),
        }
    }
}

impl FromValue<u32> for Value {
    fn fromValue(self) -> u32 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<u32>().unwrap();
                num
            }
            _ => panic!("invalid value for u32"),
        }
    }
}

impl FromValue<i32> for Value {
    fn fromValue(self) -> i32 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<i32>().unwrap();
                num
            }
            _ => panic!("invalid value for i32"),
        }
    }
}

impl FromValue<u64> for Value {
    fn fromValue(self) -> u64 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<u64>().unwrap();
                num
            }
            _ => panic!("invalid value for u64"),
        }
    }
}

impl FromValue<i64> for Value {
    fn fromValue(self) -> i64 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<i64>().unwrap();
                num
            }
            _ => panic!("invalid value for i64"),
        }
    }
}

impl FromValue<f32> for Value {
    fn fromValue(self) -> f32 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<f32>().unwrap();
                num
            }
            _ => panic!("invalid value for f32"),
        }
    }
}

impl FromValue<f64> for Value {
    fn fromValue(self) -> f64 {
        match self {
            Value::Number(n) => {
                let num = n.parse::<f64>().unwrap();
                num
            }
            _ => panic!("invalid value for f64"),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////

impl ToValue<bool> for bool {
    fn toValue(self) -> Value {
        Value::Bool(self)
    }
}

impl ToValue<String> for String {
    fn toValue(self) -> Value {
        Value::String(self.clone())
    }
}

impl ToValue<Vec<Value>> for Vec<Value> {
    fn toValue(self) -> Value {
        Value::Array(self)
    }
}

impl ToValue<HashMap<String, Value>> for HashMap<String, Value> {
    fn toValue(self) -> Value {
        Value::Object(self)
    }
}

impl ToValue<u8> for u8 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl ToValue<i8> for i8 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl ToValue<u16> for u16 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl ToValue<i16> for i16 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl ToValue<u32> for u32 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl ToValue<i32> for i32 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl ToValue<u64> for u64 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl ToValue<i64> for i64 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl ToValue<f32> for f32 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}

impl ToValue<f64> for f64 {
    fn toValue(self) -> Value {
        Value::Number(self.to_string())
    }
}
