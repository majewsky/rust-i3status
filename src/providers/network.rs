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

use fact::{Fact, FactClass, FactPriority};
use providers;

pub struct Provider {}

impl providers::Provider for Provider {

    fn id(&self) -> &'static str {
        "network"
    }

    fn exec_command(&mut self, _args: Vec<&str>) -> bool {
        false
    }

    fn render(&mut self) -> Vec<Fact> {
        //TODO: ugly
        let mut ips: Vec<String> = Vec::new();
        for interface in datalink::interfaces() {
            if interface.name == "docker0" {
                //that one is not connecting me to the internet
                continue
            }
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

        vec![Fact{
            class: FactClass::NetworkFact,
            priority: FactPriority::PassiveFact,
            text: ips.join(" "),
        }]
    }

}
