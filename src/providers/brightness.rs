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

use fact::{Fact, FactClass, FactPriority};
use providers;
use util::{read_number_from_file, write_number_to_file};

const SCREEN_MAX_PATH: &'static str = "/sys/class/backlight/intel_backlight/max_brightness";
const SCREEN_NOW_PATH: &'static str = "/sys/class/backlight/intel_backlight/brightness";

pub struct Provider {
    screen_max: Option<u64>,
}

pub fn new_provider() -> Provider {
    Provider {
        screen_max: read_number_from_file(SCREEN_MAX_PATH),
    }
}

//TODO: add keyboard brightness
impl providers::Provider for Provider {

    fn id(&self) -> &'static str {
        "brightness"
    }

    fn exec_command(&mut self, args: Vec<&str>) -> bool {
        if args.len() == 1 {
            if args[0] == "up" {
                self.adjust_screen(true);
            } else if args[0] == "down" {
                self.adjust_screen(false);
            }
        }
        false
    }

    fn render(&mut self) -> Vec<Fact> {
        let screen_percent = match self.get_screen_percent() {
            Some(val) => val,
            None      => return Vec::new(),
        };

        vec![Fact {
            class: FactClass::BrightnessFact,
            priority: FactPriority::PassiveFact,
            text: format!("{}%", screen_percent),
        }]
    }

}

impl Provider {
    fn get_screen_percent(&self) -> Option<u64> {
        Some(read_number_from_file(SCREEN_NOW_PATH)? * 100 / (self.screen_max?))
    }

    fn adjust_screen(&mut self, up: bool) {
        if let Some(percent) = self.get_screen_percent() {
            let mut percent = percent;
            if let Some(screen_max) = self.screen_max {
                //move to the next 5% increment
                if up {
                    percent += 1;
                    while percent % 5 != 0 {
                        percent += 1;
                    }
                } else {
                    percent -= 1;
                    while percent % 5 != 0 {
                        percent -= 1;
                    }
                }

                //translate to actual value
                let mut value = screen_max * percent / 100;
                if value > screen_max {
                    value = screen_max;
                }
                if value == 0 { //never let the display turn completely off
                    value = 1;
                }
                write_number_to_file(SCREEN_NOW_PATH, value).ok();
            }
        }
    }
}
