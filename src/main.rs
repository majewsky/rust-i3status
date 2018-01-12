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
extern crate libc;
extern crate pnet;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

use chrono::{Local, Timelike};
use std::io;
use std::mem;
use std::ptr::null_mut;
use std::time::Duration;

mod fact;
mod providers;
mod render;

fn main() {
    //initialize protocol
    println!("{{\"version\":1}}\n[");

    let mut providers = providers::all();
    let mut stdin_readable = false;

    loop {
        //execute command, if present
        if stdin_readable {
            let mut s = String::new();
            if let Ok(_) = io::stdin().read_line(&mut s) {
                let mut iter = s.split_whitespace();
                if let Some(command) = iter.next() {
                    for p in providers.iter_mut() {
                        if p.id() == command {
                            p.exec_command(iter.collect());
                            break;
                        }
                    }
                }
            }
        }

        //render statusline
        render::to_stdout(providers.iter_mut()
            .flat_map(|p| p.render())
            .collect());

        //sleep until next full second, but wake up when receiving user command
        let nsecs = 1_000_000_000 - (Local::now().nanosecond() % 1_000_000_000);
        stdin_readable = match wait_for_readable(libc::STDIN_FILENO, nsecs) {
            Ok(val) => val,
            Err(_) => {
                //fallback to regular sleep() if select() does not work
                let nsecs = 1_000_000_000 - (Local::now().nanosecond() % 1_000_000_000);
                std::thread::sleep(Duration::new(0, nsecs));
                false
            },
        };
    }
}

//Returns whether `fd` has become readable before the timeout.
fn wait_for_readable(fd: libc::c_int, timeout_nsecs: u32) -> io::Result<bool> {
    unsafe {
        let mut in_fds: libc::fd_set = mem::uninitialized();
        libc::FD_ZERO(&mut in_fds);
        libc::FD_SET(fd, &mut in_fds);
        let mut tv =  libc::timeval {
            tv_sec: 0,
            tv_usec: (timeout_nsecs / 1000) as libc::suseconds_t,
        };
        let retval = libc::select(1, &mut in_fds, null_mut(), null_mut(), &mut tv);
        if retval == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(libc::FD_ISSET(fd, &mut in_fds))
    }
}
