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
use string::DualString::{Dynamic,Static};

const ENERGY_FULL_PATH: &'static str = "/sys/class/power_supply/BAT0/energy_full";
const ENERGY_NOW_PATH:  &'static str = "/sys/class/power_supply/BAT0/energy_now";
const POWER_ONLINE_PATH: &'static str = "/sys/class/power_supply/AC0/online";

const CHARGING_COLOR: &'static str = "#00AA00";
const NORMAL_COLOR:   &'static str = "#AAAA00";
const WARNING_COLOR:  &'static str = "#AA0000";

pub struct Provider {}

impl block::Provider for Provider {

    fn id(&self) -> &'static str {
        "battery"
    }

    fn exec_command(&mut self, _args: Vec<&str>) -> bool {
        false
    }

    fn render(&mut self) -> Vec<Block> {
        let data = match read_battery_data() {
            Some(val) => val,
            None      => return Vec::new(),
        };

        let color = if data.is_charging {
            CHARGING_COLOR
        } else if data.energy_percent < 10 {
            WARNING_COLOR
        } else {
            NORMAL_COLOR
        };

        make_section("bat", &[
            Block{
                name: Static("battery"),
                full_text: Dynamic(format!("{}%", data.energy_percent)),
                color: Static(color),
                ..Block::default()
            },
        ])
    }

}

struct BatteryData {
    energy_percent: i64,
    is_charging: bool,
}

fn read_battery_data() -> Option<BatteryData> {
    let energy_full = read_number_from_file(ENERGY_FULL_PATH)?;
    let energy_now = read_number_from_file(ENERGY_NOW_PATH)?;
    let power_online = read_number_from_file(POWER_ONLINE_PATH)?;
    return Some(BatteryData {
        energy_percent: energy_now * 100 / energy_full,
        is_charging: power_online > 0,
    });
}

fn read_number_from_file(path: &str) -> Option<i64> {
    let mut file = File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    contents.trim().parse().ok()
}
