/*
 * Copyright (C) 2020-2022 Fanout, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub mod app;
pub mod arena;
pub mod buffer;
pub mod channel;
pub mod connection;
pub mod event;
pub mod executor;
pub mod future;
pub mod http1;
pub mod list;
pub mod listener;
pub mod net;
pub mod reactor;
pub mod resolver;
pub mod server;
pub mod shuffle;
pub mod timer;
pub mod tls;
pub mod tnetstring;
pub mod waker;
pub mod websocket;
pub mod zhttppacket;
pub mod zhttpsocket;
pub mod zmq;

use app::Config;
use log::info;
use std::error::Error;

// from futures-rs
#[macro_export]
macro_rules! pin_mut {
    ($($x:ident),* $(,)?) => { $(
        // Move the value to ensure that it is owned
        let mut $x = $x;
        // Shadow the original binding so that it can't be directly accessed
        // ever again.
        #[allow(unused_mut)]
        let mut $x = unsafe {
            Pin::new_unchecked(&mut $x)
        };
    )* }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    info!("starting...");

    {
        let a = match app::App::new(config) {
            Ok(a) => a,
            Err(e) => {
                return Err(e.into());
            }
        };

        info!("started");

        a.wait_for_term();

        info!("stopping...");
    }

    info!("stopped");

    Ok(())
}
