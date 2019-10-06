#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread;
    use super::*;

    #[test]
    fn test_get() {
        assert_eq!(add(1, 1).item, 1);
        assert_eq!(add(1, 2).item, 2);
        assert_eq!(get(1).len(), 2);
    }

    #[test]
    fn test_delete() {
        let secs = Duration::from_secs(1);

        assert_eq!(add_time(1, 1, 1).item, 1);
        thread::sleep(secs);
        assert_eq!(add_time(1, 2, 1).item, 2);
        thread::sleep(secs);
        delete(1, 1);
        assert_eq!(get(1).len(), 1);
        delete(1, 2);
        assert_eq!(get(1).len(), 0);
    }
}

// table structure
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Order {
    pub table: i32,
    // use this as index
    #[serde(default)]
    pub item: i32,
    #[serde(skip)]
    time: u64,
}


// indexing for fast look up
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<i32,Vec<Order>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}
const OFFSET: u64 = 300;

pub fn get(t: i32) -> Vec<Order> {
    let m = HASHMAP.lock().unwrap();
    if !m.contains_key(&t) {
        return vec![];
    }
    return m[&t].to_owned();
}

pub fn delete(t: i32, i: i32) {
    let mut m = HASHMAP.lock().unwrap();
    if !m.contains_key(&t) {
        return;
    }
    let mut c = m[&t].to_owned();
    let time = get_time();
    c = c.into_iter().filter(|x| x.time >= time && x.item != i).collect();
    *m.entry(t).or_insert(Vec::new()) = c.to_owned();
    println!("Delete success for {}.", t);
}


pub fn get_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH);
    return since_the_epoch.unwrap().as_secs(); // 5 minute
}

fn add_time(t: i32, i: i32, o: u64) -> Order {
    let mut hm = HASHMAP.lock().unwrap();
    let d = Order { table: t, item: i, time: get_time() + o };
    hm.entry(t).or_insert_with(Vec::new).push(d);
    println!("Created new object {:?}.", d);
    d
}

pub fn add(t: i32, i: i32) -> Order {
    add_time(t, i, OFFSET)
}