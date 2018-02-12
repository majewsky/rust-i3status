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

extern crate chrono;
extern crate ipnetwork;
#[macro_use]
extern crate json;
extern crate libc;
extern crate pnet;

use std::sync::mpsc;

mod fact;
mod providers;
mod render;
mod threads;
mod util;

fn main() {
    //initialize protocol
    println!("{{\"version\":1}}\n[");

    //initialize providers
    let mut providers = providers::all();

    //launch threads
    let (tx, rx) = mpsc::channel();
    threads::wait_for_input(tx.clone());
    threads::periodic_wakeup(tx.clone());

    loop {
        //wait for next event
        match rx.recv().unwrap() {
            threads::Event::Render => {},
            threads::Event::Command(s) => {
                //execute received command
                let mut iter = s.split_whitespace();
                if let Some(command) = iter.next() {
                    for p in providers.iter_mut() {
                        if p.id() == command {
                            p.exec_command(iter.collect());
                            break;
                        }
                    }
                }
            },
        }

        //render statusline
        render::to_stdout(providers.iter_mut()
            .flat_map(|p| p.render())
            .collect());
    }
}
