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

#[derive(PartialEq,Eq)]
pub enum FactClass {
    NetworkFact,
    BrightnessFact,
    VolumeFact,
    BatteryFact,
}

impl FactClass {
    pub fn all() -> [FactClass;4] {
        [FactClass::NetworkFact,FactClass::BrightnessFact,FactClass::VolumeFact,FactClass::BatteryFact]
    }

    pub fn label(&self) -> &'static str {
        match *self {
            FactClass::NetworkFact => "net",
            FactClass::BrightnessFact => "light",
            FactClass::VolumeFact => "vol",
            FactClass::BatteryFact => "bat",
        }
    }
}

#[derive(PartialEq,Eq)]
pub enum FactPriority {
    PositiveFact,
    PassiveFact,
    // Warning, //for future expansion
    DangerFact,
}

impl FactPriority {
    pub fn color(&self) -> &'static str {
        match *self {
            FactPriority::PositiveFact => "#00AA00",
            FactPriority::PassiveFact => "#00AAAA",
            // FactPriority::Warning => "#AAAA00",
            FactPriority::DangerFact => "#AA0000",
        }
    }
}

pub struct Fact {
    pub class: FactClass,
    pub priority: FactPriority,
    pub text: String,
}
