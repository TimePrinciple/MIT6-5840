#![allow(non_snake_case)]

use std::{env, fs::read_to_string};

use anyhow::Result;
use libloading::{Library, Symbol};
use MIT6_5840::LEC1::mr::worker::KeyValue;

type Map = fn(String, String) -> Vec<KeyValue>;
type Reduce = fn(String, Vec<String>) -> String;

fn main() -> Result<()> {
    mr_sequential()?;

    Ok(())
}

fn mr_sequential() -> Result<()> {
    if env::args().len() < 3 {
        eprintln!("Usage: me_sequential <plugin>.so <input_files>");
    }

    let (map, reduce, _lib) = load_plugin(env::args().nth(1).unwrap()).unwrap();

    // Read each input file, pass it to Map, accumulate the intermediate Map
    // output.
    let mut intermediate = Vec::<KeyValue>::new();
    let files = env::args().skip(2);
    for filename in files {
        let content = read_to_string(&filename)?;
        let kva = map(filename, content);
        intermediate.extend(kva);
    }

    // A big difference from read MapReduce is that all the intermediate data is in
    // one place, `intermediate`, rather than being partitioned in N*M buckets.
    intermediate.sort_by_key(|pair| pair.key.clone());

    // Call reduce on each distinct key in `intermediate`
    let mut i = 0;
    while i < intermediate.len() {
        let mut j = i + 1;
        while j < intermediate.len() && intermediate[j].key == intermediate[i].key {
            j += 1;
        }
        let mut values = Vec::<String>::new();
        for k in i..j {
            values.push(intermediate[k].value.to_string());
        }
        let output = reduce(intermediate[i].key.to_string(), values);

        // Format for each line of Reduce output
        println!("{} {}", intermediate[i].key, output);

        i = j
    }

    Ok(())
}

fn load_plugin(filename: String) -> Result<(Map, Reduce, Library)> {
    unsafe {
        let plugin = Library::new(filename)?;
        let map: Symbol<Map> = plugin.get(b"map")?;
        let reduce: Symbol<Reduce> = plugin.get(b"reduce")?;
        // Library should be move out along with functions, otherwise these functions
        // would be lost when Library is dropped
        Ok((*map, *reduce, plugin))
    }
}
