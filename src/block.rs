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

fn is_zero(x: &u32) -> bool {
    *x == 0
}

#[derive(Default,Serialize)]
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
