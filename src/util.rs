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

use std::fs::File;
use std::io::{Read, Result, Write};

pub fn read_number_from_file(path: &str) -> Option<u64> {
    let mut file = File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    contents.trim().parse().ok()
}

pub fn write_number_to_file(path: &str, number: u64) -> Result<()> {
    let mut file = File::create(path)?;
    write!(file, "{}", number)
}
