use serde_json::Value;
use std::io;

fn main() {
    let mut iter = io::stdin().lines().peekable();
    while let Some(mut log_stmt) = iter.next() {
        let v: Value = serde_json::from_str(&log_stmt.unwrap()).unwrap();
        let mut body = v["body"].to_string();
        //let mut log_stmt = serde_json::from_str()log_stmt.unwrap();
        while let Some(line) = iter.next_if(|stmt2| {
            if let Ok(line) = stmt2 {
                let v_next: Value = serde_json::from_str(&line).unwrap();
                if v_next["multi"].as_bool().unwrap() {
                    body.push_str(&v_next["body"].to_string());
                    true
                } else {
                    false
                }
            } else {
                panic!("an error shouldn't have ocurred here");
            }
        }) {
            // Basically noop, since we already pushed the content to the new object inside
            // of `.next_if` in order to avoid having to parse the json twice
            // TODO: benchmark this against the "other" implementation to get an idea of the
            //   performance hit we take
        }
        println!("{}", body);
    }
}
