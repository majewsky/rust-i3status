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

use serde::ser::{Serialize, Serializer};

#[derive(Clone)]
pub enum DualString {
    Static(&'static str),
    Dynamic(String),
}

impl DualString {
    pub fn is_empty(&self) -> bool {
        match *self {
            DualString::Static(ref s) => s.is_empty(),
            DualString::Dynamic(ref s) => (&s).is_empty(),
        }
    }
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
