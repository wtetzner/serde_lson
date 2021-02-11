
extern crate serde_lson;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap,BTreeMap};
use maplit::btreemap;

fn main() {
    let metadata = Metadata {
        labels: btreemap! {
            "FOO".to_string() => Label {
                address: 0x34,
                description: Some("The FOO function".to_string())
            },
            "function".to_string() => Label {
                address: 0x56,
                description: Some("The BAR function".to_string())
            }
        },
        vars: btreemap! {
            "ACC".to_string() => Variable {
                name: Some("Accumulator".to_string()),
                description: Some("Accumulates shit".to_string()),
                address: 0x100,
                bits: None
            },
            "PSW".to_string() => Variable {
                name: Some("Processor Status Word".to_string()),
                description: Some("Status flags".to_string()),
                address: 0x101,
                bits: Some(btreemap! {
                    "P".to_string() => Bitfield {
                        bit: 0,
                        name: Some("Parity".to_string()),
                        description: Some("This bit is set when the number of set bits in ACC is odd. Read only.".to_string()),
                        readonly: true
                    }
                })
            }
        }
    };

    println!("{}", serde_lson::ser::to_string_pretty(&metadata).unwrap());
    println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
    println!("{}", toml::to_string(&metadata).unwrap());

    let mut map = HashMap::new();

    map.insert(Blah { foo: 45 }, "blah");
    println!("{}", serde_lson::ser::to_string_pretty(&map).unwrap());


    serde_lson::parse_str::<String>("and nil -128 321 elseif true false");
}

#[derive(Debug,Clone,Deserialize,Serialize,Hash,PartialEq,Eq)]
pub struct Blah {
    foo: usize
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Metadata {
    labels: BTreeMap<String,Label>,
    vars: BTreeMap<String,Variable>
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Label {
    address: i32,
    description: Option<String>
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Variable {
    name: Option<String>,
    description: Option<String>,
    address: i32,
    bits: Option<BTreeMap<String,Bitfield>>
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Bitfield {
    bit: u8,
    name: Option<String>,
    description: Option<String>,
    readonly: bool
}
