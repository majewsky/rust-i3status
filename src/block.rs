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

use std::ops::Not;
use std::vec::Vec;

fn is_zero(x: &u32) -> bool {
    *x == 0
}

#[derive(Clone,Default,Serialize)]
pub struct Block<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<&'a str>,
    pub full_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text: Option<String>,
    #[serde(skip_serializing_if = "str::is_empty")]
    pub color: &'a str,
    #[serde(rename = "background", skip_serializing_if = "str::is_empty")]
    pub background_color: &'a str,
    #[serde(skip_serializing_if = "Not::not")]
    pub urgent: bool,
    pub separator: bool,
    #[serde(skip_serializing_if = "is_zero")]
    pub separator_block_width: u32,
}

pub trait Provider {
    fn render(&self) -> Vec<Block>;
}

pub fn make_section<'a>(caption: &'static str, blocks: &[Block<'a>]) -> Vec<Block<'a>>{
    if blocks.is_empty() {
        return Vec::new();
    }

    //add a header in front of the given blocks, while trying to match the color
    //of the existing blocks
    let first_block = &(blocks[0]);
    let mut result = vec![
        Block {
            name: first_block.name,
            instance: Some("_caption"),
            full_text: caption.to_owned(),
            color: first_block.color,
            ..Block::default()
        },
    ];
    result.extend_from_slice(&blocks);

    //add a separator to the right of the rightmost block to visually separate sections
    {
        let last_idx = result.len() - 1;
        let last_block = &mut result[last_idx];
        last_block.separator = true;
        last_block.separator_block_width = 15;
    }

    result
}
