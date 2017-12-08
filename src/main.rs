/*******************************************************************************
*
* Copyright 2017 Stefan Majewsky <majewsky@gmx.net>
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

extern crate chrono;
extern crate ipnetwork;
extern crate pnet;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

use chrono::{Local, Timelike};
use std::time::Duration;

mod block;
mod providers;
use block::Block;

fn main() {
    //initialize protocol
    println!("{{\"version\":1}}\n[");

    let providers = providers::all();

    loop {
        //collect blocks from all providers
        let blocks: Vec<Block> = providers.iter()
            .flat_map(|p| p.render())
            .collect();

        //show blocks
        println!("{},", json!(blocks).to_string());

        //sleep until next full second
        let nsecs = 1_000_000_000 - (Local::now().nanosecond() % 1_000_000_000);
        std::thread::sleep(Duration::new(0, nsecs));
    }
}
