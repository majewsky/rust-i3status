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

pub mod battery;
pub mod network;

use fact;

pub trait Provider {
    fn id(&self) -> &'static str;
    fn exec_command(&mut self, args: Vec<&str>) -> bool;
    fn render(&mut self) -> Vec<fact::Fact>;
}

pub fn all() -> Vec<Box<Provider>> {
    vec![
        Box::new(network::Provider{}),
        Box::new(battery::Provider{}),
    ]
}
