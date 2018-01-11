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

use ipnetwork::IpNetwork;
use pnet::datalink;

use block;
use block::{Block, make_section};
use string::DualString::{Dynamic,Static};

pub struct Provider {}

impl block::Provider for Provider {

    fn id(&self) -> &'static str {
        "network"
    }

    fn exec_command(&mut self, _args: Vec<&str>) -> bool {
        false
    }

    fn render(&mut self) -> Vec<Block> {
        //TODO: ugly
        let mut ips: Vec<String> = Vec::new();
        for interface in datalink::interfaces() {
            for ip_net in interface.ips {
                match ip_net {
                    IpNetwork::V6(_) => continue,
                    IpNetwork::V4(ip_net) => {
                        let ip = ip_net.ip();
                        if ip.is_loopback() {
                            continue
                        }
                        if ip.is_link_local() {
                            continue
                        }
                        ips.push(format!("{}", ip))
                    },
                }
            }
        }
        if ips.len() == 0 {
            return vec![];
        }
        ips.sort();

        make_section("ip", &[
            Block{
                name: Static("network"),
                full_text: Dynamic(ips.join(" ")),
                color: Static("#00AAAA"),
                ..Block::default()
            },
        ])
    }

}
