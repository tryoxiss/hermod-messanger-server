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

use std::net::TcpStream;
use std::io::prelude::*;

pub fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    warning!("Stream is read and then unwrapped! Don't unwrap!");

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    warning!("Stream is written to and then unwrapped! Don't unwrap!");

    stream.flush().unwrap();
    warning!("Stream flush is unwrapped! Don't unwrap!");
}