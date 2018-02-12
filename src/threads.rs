/******************************************************************************
*
*  Copyright 2018 Stefan Majewsky <majewsky@gmx.net>
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
*
******************************************************************************/

use chrono::{Local, Timelike};
use std::io;
use std::sync::mpsc;
use std::time::Duration;
use std::thread;

pub enum Event {
  //EventRender does nothing except alerting the main thread to re-render the statusbar.
  Render,
  //EventCommand executes the given command (which is parsed in the main thread).
  Command(String),
}

pub fn wait_for_input(sink: mpsc::Sender<Event>) {
    thread::spawn(move || {
        loop {
            let mut s = String::new();
            if let Ok(_) = io::stdin().read_line(&mut s) {
                sink.send(Event::Command(s)).unwrap();
            }
        }
    });
}

pub fn periodic_wakeup(sink: mpsc::Sender<Event>) {
    thread::spawn(move || {
        //make sure that first render happens immediately
        sink.send(Event::Render).unwrap();
        //re-render on every full second (so that the clock is as precise as possible)
        loop {
            let nsecs = 1_000_000_000 - (Local::now().nanosecond() % 1_000_000_000);
            thread::sleep(Duration::new(0, nsecs));
            sink.send(Event::Render).unwrap();
        }
    });
}
