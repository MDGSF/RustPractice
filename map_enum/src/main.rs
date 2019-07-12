use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum Value {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.format_value(self, 0))
    }
}

impl Value {
    fn format_value(&self, input: &Value, level: i32) -> String {
        match input {
            Value::Null => self.format_null(),
            Value::Bool(b) => self.format_bool(*b),
            Value::Number(n) => self.format_number(n.to_string()),
            Value::String(s) => self.format_string(s.to_string()),
            Value::Array(a) => self.format_array(a, level),
            Value::Object(o) => self.format_object(o, level),
        }
    }

    fn format_null(&self) -> String {
        "null".to_string()
    }

    fn format_bool(&self, input: bool) -> String {
        if input {
            return "true".to_string();
        }
        "false".to_string()
    }

    fn format_number(&self, input: String) -> String {
        input
    }

    fn format_string(&self, input: String) -> String {
        "\"".to_string() + &input + "\""
    }

    fn format_array(&self, input: &Vec<Value>, level: i32) -> String {
        let mut result = "[\n".to_string();

        let mut idx = 0;
        let length = input.len();
        for item in input.iter() {
            result += " ".repeat(((level + 1) * 4) as usize).as_str();
            result += self.format_value(item, level + 1).as_str();

            if idx == length - 1 {
                result += "\n";
            } else {
                result += ",\n";
            }

            idx += 1;
        }

        result += " ".repeat((level * 4) as usize).as_str();
        result += "]";
        result
    }

    fn format_object(&self, input: &HashMap<String, Value>, level: i32) -> String {
        let mut result = "{\n".to_string();

        let mut idx = 0;
        let length = input.len();
        for (k, v) in input {
            result += " ".repeat(((level + 1) * 4) as usize).as_str();
            result = result + "\"" + &k + "\": ";
            result += self.format_value(&v, level + 1).as_str();

            if idx == length - 1 {
                result += "\n";
            } else {
                result += ",\n";
            }

            idx += 1;
        }

        result += " ".repeat((level * 4) as usize).as_str();
        result += "}";
        result
    }
}

fn main() {
    let mut sub: HashMap<String, Value> = HashMap::new();
    sub.insert("Null".to_string(), Value::Null);

    let mut m: HashMap<String, Value> = HashMap::new();
    m.insert("I'm Null".to_string(), Value::Null);
    m.insert("I'm Bool(bool)".to_string(), Value::Bool(true));
    m.insert(
        "I'm Number(String)".to_string(),
        Value::Number("123".to_string()),
    );
    m.insert(
        "I'm String(String)".to_string(),
        Value::String("String Value".to_string()),
    );
    m.insert(
        "I'm Array(Vec<Value>)".to_string(),
        Value::Array(vec![
            Value::Number("1".to_string()),
            Value::Number("2".to_string()),
            Value::Number("3".to_string()),
        ]),
    );

    m.insert(
        "I'm Object(HashMap<String, Value>)".to_string(),
        Value::Object(sub),
    );

    let obj: Value = Value::Object(m);

    println!("{}", obj);
}
