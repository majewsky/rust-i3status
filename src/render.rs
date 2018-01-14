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
use json;
use std::vec::Vec;

use fact::{Fact, FactClass, FactPriority};

////////////////////////////////////////////////////////////////////////////////
// struct Block

#[derive(Clone,Default)]
struct Block {
    name: &'static str,
    instance: Option<&'static str>,
    full_text: DualString,
    short_text: Option<&'static str>,
    color: &'static str,
    urgent: bool,
    separator: bool,
    separator_block_width: u32,
}

impl Block {
    fn to_json(&self) -> json::JsonValue {
        let mut obj = object!{
            "name" => self.name,
            "full_text" => self.full_text.as_ref(),
            "separator" => self.separator,
        };
        if let Some(val) = self.instance {
            obj["instance"] = val.into();
        }
        if let Some(val) = self.short_text {
            obj["short_text"] = val.into();
        }
        if !self.color.is_empty() {
            obj["color"] = self.color.into();
        }
        if self.urgent {
            obj["urgent"] = true.into();
        }
        if self.separator_block_width > 0 {
            obj["separator_block_width"] = self.separator_block_width.into();
        }
        obj
    }
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

impl AsRef<str> for DualString {
    fn as_ref(&self) -> &str
    {
        match *self {
            DualString::Static(ref s) => s,
            DualString::Dynamic(ref s) => &s,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// rendering

pub fn to_stdout(facts: Vec<Fact>) {
    let mut blocks = compile_facts(facts);
    blocks.extend_from_slice(&render_clock());
    println!("{},", json::JsonValue::Array(blocks.iter().map(Block::to_json).collect()));
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
