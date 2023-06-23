/* 
 *      This file is part of:
 *      Codename Bonfire Instant Messanger
 *
 *      Copyright (C) 2023 or later - Project Bonfire Contributers
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 * 
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

// use crate::terminal_out;

use log::{debug, error, info, trace, warn};
use log4rs;

use std::net::TcpStream;
use std::io::prelude::*;

pub fn handle_connection(mut stream: TcpStream)
{
    warn!("The `handle_connection` TCP Stream buffer is a defined size 
             of 1024 bytes. This should be able to handle a stream 
             of any size.");
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    warn!("Stream is read and then unwrapped! Don't unwrap!");

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    warn!("Stream is written to and then unwrapped! Don't unwrap!");

    stream.flush().unwrap();
    warn!("Stream flush is unwrapped! Don't unwrap!");
}