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

use std::fs::File;
use std::io::Read;

use block;
use block::{Block, make_section};

const ENERGY_FULL_PATH: &'static str = "/sys/class/power_supply/BAT0/energy_full";
const ENERGY_NOW_PATH:  &'static str = "/sys/class/power_supply/BAT0/energy_now";
const POWER_ONLINE_PATH: &'static str = "/sys/class/power_supply/AC0/online";

const CHARGING_COLOR: &'static str = "#00AA00";
const NORMAL_COLOR:   &'static str = "#AAAA00";
const WARNING_COLOR:  &'static str = "#AA0000";

pub struct Provider {}

impl block::Provider for Provider {

    fn render(&self) -> Vec<Block> {
        //TODO simplify according to suggestions in https://stackoverflow.com/a/47650277/334761
        //when Rust 1.22 enters Arch Linux [community]
        let energy_full = match read_number_from_file(ENERGY_FULL_PATH) {
            Some(val) => val,
            None      => return Vec::new(),
        };
        let energy_now = match read_number_from_file(ENERGY_NOW_PATH) {
            Some(val) => val,
            None      => return Vec::new(),
        };
        let is_charging = match read_number_from_file(POWER_ONLINE_PATH) {
            Some(val) => val > 0,
            None      => return Vec::new(),
        };

        let energy_percent = energy_now * 100 / energy_full;
        let color = if is_charging {
            CHARGING_COLOR
        } else if energy_percent < 10 {
            WARNING_COLOR
        } else {
            NORMAL_COLOR
        };

        make_section("bat", &[
            Block{
                name: "battery",
                full_text: format!("{}%", energy_percent),
                color: color,
                ..Block::default()
            },
        ])
    }

}

fn read_number_from_file(path: &str) -> Option<i64> {
    //TODO use `?` operator instead of `match` after Rust 1.22 enters Arch Linux [community]
    let mut file = match File::open(path).ok() {
        Some(f) => f,
        None => return None,
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents).ok() {
        Some(_) => {},
        None => return None,
    };
    contents.trim().parse::<i64>().ok()
}
