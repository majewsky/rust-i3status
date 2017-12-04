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

use chrono::{Datelike, Local, Timelike};

use block;
use block::Block;

pub struct Provider {}

impl block::Provider for Provider {

    fn render(&self) -> Vec<Block> {
        let now = Local::now();
        let date = format!("{:04}-{:02}-{:02}", now.year(), now.month(), now.day());
        let time = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        vec![
            Block{
                name: "clock",
                instance: Some("date"),
                full_text: date,
                short_text: Some(" ".to_owned()),
                color: "#AAAAAA",
                separator_block_width: 6,
                ..Block::default()
            },
            Block{
                name: "clock",
                instance: Some("time"),
                full_text: time,
                ..Block::default()
            },
        ]
    }

}
