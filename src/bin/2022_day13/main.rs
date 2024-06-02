use rusty_xmas;
use serde_json::{self, json, Value};
use std::cmp::Ordering;

fn main() {
    let input = rusty_xmas::load_input!();
    let data: Vec<&str> = input.split_ascii_whitespace().collect();
    let mut packets: Vec<Value> = data
        .into_iter()
        .map(|packet| serde_json::from_str(packet).unwrap())
        .collect();

    let mut answer = 0;
    for (index, packet) in packets.chunks(2).enumerate() {
        let ord = compare_json(&packet[0], &packet[1]);
        // dbg!(ord);
        match ord {
            Ordering::Less => {
                answer += index + 1;
            }
            Ordering::Greater => (),
            Ordering::Equal => unreachable!(),
        }
    }
    println!("Part 1: {}", answer);

    packets.extend(vec![json!([[2]]), json!([[6]])]);
    packets.sort_by(|a, b| compare_json(a, b));
    let divider_1 = packets.iter().position(|a| *a == json!([[2]])).unwrap() + 1;
    let divider_2 = packets.iter().position(|a| *a == json!([[6]])).unwrap() + 1;
    println!("Part 2: {}", divider_1 * divider_2);
}

fn compare_json(value1: &Value, value2: &Value) -> std::cmp::Ordering {
    match (value1, value2) {
        (Value::Number(n1), Value::Number(n2)) => {
            let f1 = n1.as_u64().unwrap();
            let f2 = n2.as_u64().unwrap();
            f1.partial_cmp(&f2).unwrap()
        }
        (Value::Array(arr1), Value::Array(arr2)) => {
            for (v1, v2) in arr1.iter().zip(arr2.iter()) {
                let ord = compare_json(v1, v2);
                if ord != std::cmp::Ordering::Equal {
                    return ord;
                }
            }
            arr1.len().cmp(&arr2.len())
        }
        (Value::Array(_), Value::Number(_)) => {
            compare_json(value1, &Value::Array(vec![value2.clone()]))
        }
        (Value::Number(_), Value::Array(_)) => {
            compare_json(&Value::Array(vec![value1.clone()]), value2)
        }
        _ => Ordering::Equal,
    }
}
