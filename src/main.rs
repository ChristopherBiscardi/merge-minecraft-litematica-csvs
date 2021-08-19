use std::collections::HashMap;

use eyre::{Result, WrapErr};
use serde::Deserialize;

#[derive(Deserialize)]
struct Item {
    #[serde(rename = "Item")]
    name: String,
    #[serde(rename = "Total (x5)")]
    number: u32,
}
fn main() -> Result<()> {
    // Build the CSV reader and iterate over each record.
    let mut rdr =
        csv::Reader::from_path("./litematica-1.csv")?;
    let items_1 = rdr
        .deserialize()
    .collect::<std::result::Result<Vec<Item>, csv::Error>>()?;

    let mut rdr =
        csv::Reader::from_path("./litematica-2.csv")?;
    let items_2 = rdr
        .deserialize()
        .collect::<std::result::Result<Vec<Item>, csv::Error>>()?;

    let mut map = items_1
        .into_iter()
        .map(|Item { name, number }| (name, number))
        .collect::<HashMap<String, u32>>();

    for Item { name, number } in items_2.iter() {
        map.entry(name.to_string())
            .and_modify(|original_number| {
                *original_number += number
            })
            .or_insert(*number);
    }

    println!("name,amount");
    for (key, value) in map.iter() {
        println!("{},{}", key, value);
    }

    Ok(())
}
