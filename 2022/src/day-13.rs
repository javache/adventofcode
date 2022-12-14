use itertools::Itertools;
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::io::{self, Read};

fn cmp(left: &serde_json::Value, right: &serde_json::Value) -> Ordering {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.as_i64().cmp(&right.as_i64()),
        (Value::Array(left), Value::Array(right)) => {
            // TODO: if left contained a copy we could implement Ord on, this would just be
            // standard list comparison
            for i in 0..(left.len().max(right.len())) {
                match (left.get(i), right.get(i)) {
                    (Some(l), Some(r)) => {
                        let res = cmp(l, r);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    (None, Some(_)) => {
                        return Ordering::Less;
                    }
                    (Some(_), None) => {
                        return Ordering::Greater;
                    }
                    (None, None) => {}
                }
            }
            Ordering::Equal
        }
        (Value::Number(_), right) => cmp(&Value::Array(vec![left.clone()]), right),
        (left, Value::Number(_)) => cmp(left, &Value::Array(vec![right.clone()])),
        _ => panic!("Unsupported value {:?}", (left, right)),
    }
}

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    // Init with divider packets
    let mut all_packets = vec![json!([[2]]), json!([[6]])];
    let mut ordered_pairs = 0;
    for (i, (left, right)) in input
        .split("\n\n")
        .flat_map(|block| block.split("\n").collect_tuple())
        .enumerate()
    {
        let left_packet = serde_json::from_str(left)?;
        let right_packet = serde_json::from_str(right)?;
        if cmp(&left_packet, &right_packet) != Ordering::Greater {
            ordered_pairs += i + 1;
        }
        all_packets.push(left_packet);
        all_packets.push(right_packet);
    }
    println!("(1) Sum of indices of ordered pairs is {}", ordered_pairs);

    all_packets.sort_by(|a, b| cmp(a, b));
    let index_1 = all_packets.iter().position(|p| p == &json!([[2]])).unwrap();
    let index_2 = all_packets.iter().position(|p| p == &json!([[6]])).unwrap();
    println!("(2) The decoder key is {}", (index_1 + 1) * (index_2 + 1));

    Ok(())
}
