/*******************************************************************************
*
* Copyright 2018 Stefan Majewsky <majewsky@gmx.net>
*
* This program is free software: you can redistribute it and/or modify it under
* the terms of the GNU General Public License as published by the Free Software
* Foundation, either version 3 of the License, or (at your option) any later
* version.
*
* This program is distributed in the hope that it will be useful, but WITHOUT ANY
* WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
* A PARTICULAR PURPOSE. See the GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License along with
* this program. If not, see <http://www.gnu.org/licenses/>.
*
*******************************************************************************/

use chrono::{Datelike, Local, Timelike};
use serde::ser::{Serialize, Serializer};
use std::ops::Not;
use std::vec::Vec;

use fact::{Fact, FactClass, FactPriority};

////////////////////////////////////////////////////////////////////////////////
// struct Block

fn is_zero(x: &u32) -> bool {
    *x == 0
}

#[derive(Clone,Default,Serialize)]
struct Block {
    name: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<&'static str>,
    full_text: DualString,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_text: Option<&'static str>,
    #[serde(skip_serializing_if = "str::is_empty")]
    color: &'static str,
    #[serde(skip_serializing_if = "Not::not")]
    urgent: bool,
    separator: bool,
    #[serde(skip_serializing_if = "is_zero")]
    separator_block_width: u32,
}

////////////////////////////////////////////////////////////////////////////////
// struct DualString (like String, but avoids strdup of &'static str)

#[derive(Clone)]
enum DualString {
    Static(&'static str),
    Dynamic(String),
}

impl Default for DualString {
    fn default() -> DualString {
        DualString::Static("")
    }
}

impl Serialize for DualString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            DualString::Static(ref s) => serializer.serialize_str(s),
            DualString::Dynamic(ref s) => serializer.serialize_str(&s),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// rendering

pub fn to_stdout(facts: Vec<Fact>) {
    let mut blocks = compile_facts(facts);
    blocks.extend_from_slice(&render_clock());
    println!("{},", json!(blocks).to_string());
}

fn compile_facts(facts: Vec<Fact>) -> Vec<Block> {
    //avoid allocations (+10 should be enough to accommodate
    //section headers and date/time)
    let mut blocks: Vec<Block> = Vec::new();
    blocks.reserve(facts.len() + 10);

    //render one section of blocks for each fact class
    for class in FactClass::all().iter() {
        let label = class.label();
        //do not add the heading to `blocks` yet! only when we know that we
        //actually have facts in this section
        let mut heading = Some(Block {
            name: label,
            full_text: DualString::Static(label),
            color: "#AAAAAA",
            ..Block::default()
        });

        for fact in facts.iter() {
            if fact.class == *class {
                //put heading block before first fact block
                if let Some(heading_block) = heading {
                    blocks.push(heading_block);
                    heading = None;
                }
                //generate fact block
                blocks.push(Block {
                    name: label,
                    full_text: DualString::Dynamic(fact.text.clone()),
                    color: fact.priority.color(),
                    urgent: fact.priority == FactPriority::DangerFact,
                    ..Block::default()
                });
            }
        }

        //put separator after last fact block if there are any
        if heading.is_none() {
            let last_idx = blocks.len() - 1;
            let last_block = &mut blocks[last_idx];
            last_block.separator = true;
            last_block.separator_block_width = 15;
        }
    }

    blocks
}

fn render_clock() -> [Block;2] {
    let now = Local::now();
    let date = format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day());
    let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
    [
        Block{
            name: "clock",
            instance: Some("date"),
            full_text: DualString::Dynamic(date),
            short_text: Some(" "),
            color: "#AAAAAA",
            separator_block_width: 6,
            ..Block::default()
        },
        Block{
            name: "clock",
            instance: Some("time"),
            full_text: DualString::Dynamic(time),
            color: "#FFFFFF",
            ..Block::default()
        },
    ]
}
