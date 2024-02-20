#![allow(non_snake_case)]

use anyhow::Result;
use MIT6_5840::LEC1::mr::worker::KeyValue;

type Map = fn(String, String) -> Vec<String>;
type Reduce = fn(String, Vec<String>) -> String;

fn main() -> Result<()> {
    let (map, reduce) = load_plugin("target/debug/libwc.so")?;

    Ok(())
}

fn load_plugin(filename: &str) -> Result<(Map, Reduce)> {
    unsafe {
        let plugin = libloading::Library::new(filename)?;
        let map: libloading::Symbol<Map> = plugin.get(b"map")?;
        let reduce: libloading::Symbol<Reduce> = plugin.get(b"reduce")?;
        Ok((*map, *reduce))
    }
}
