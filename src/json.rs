use serde::Serialize;
use serde_json::{Map, Value};

fn sort_json_value(v: Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut entries: Vec<(String, Value)> = map.into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));

            let mut new_map = Map::with_capacity(entries.len());
            for (k, val) in entries {
                new_map.insert(k, sort_json_value(val));
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(sort_json_value).collect()),
        _ => v,
    }
}

pub fn to_sorted_json<T: Serialize>(value: &T) -> String {
    let v = serde_json::to_value(value).expect("serialize");
    serde_json::to_string(&sort_json_value(v)).expect("stringify")
}
