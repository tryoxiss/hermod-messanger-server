/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 *  This file is part of:         https://github.com/tryoxiss/bonfire-server *
 *  Hermod Messanger                          https://en.ourdomain.ext/docs/ *
 *                                                                           *
 *  Copyright (C) 2023—present : Hermod Messenger Contributers (AUTHORS.md)  *
 *                                                                           *
 *  This program is free software: you can redistribute it and/or modify     *
 *  it under the terms of the GNU Affero General Public License version 3    *
 *  as published by the Free Software Foundation.                            *
 *                                                                           *
 *  This program is distributed in the hope that it will be useful,          *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of           *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the            *
 *  GNU Affero General Public License version 3 for more details.            *
 *                                                                           *
 *  You should have received a copy of the GNU Affero General Public License *
 *  along with this program.  If not, see:                                   *
 *    https://www.gnu.org/licenses/agpl-3.0                                  *
 *                                                                           *
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

mod connection_handler;
mod startup;

// Make our own logging system, with messaghes being *what is it doing* and not *what level is it*
#[macro_use]
mod terminal_out;

mod threading;
use threading::ThreadPool;

use std::net::TcpListener;

// Make our own logging system, with messaghes being *what is it doing* and not *what level is it*
use log::trace;
use log::debug;
use log::info;
// use log::warn;
// use log::error;

static CODE_START: &str = "\x1b[40m";
static ENDBLOCK: &str   = "\x1b[0m";
static INDENT: &str     = "             ";
static BOLD: &str       = "\x1b[1m";
static UNDERLINE: &str  = "\x1b[4m";

// - (margin) = INDENT
// = (padding) = 1ch left, 1ch right
// C (content) = 1ch
//                         -------------=C=
static UL_ITEM: &str    = "              • ";

/// The main function contains all initalisation steps, aswell as the main
/// program loop.
fn main()
{
    // 🚩 FIXME: The main log file needs to be renammed to when it was run
    // once a new file is created/after the program ends, so that recent.log
    // can take its place instead of appending new logs to recent.log
    startup::init_log4rs_config();

    info!("Initalising Program");

    const REPOSITORY: &str = "tryoxiss.github.io";
    let server_version: String = startup::check_updates(
        0,
        3,
        5, 
        "hermod_server",
        "pre-release",
        4);

    startup::verify_file_integrity(&server_version, REPOSITORY);

    // config variables

    // READ CONFIG FILE
    // ASSIGN VALUES
    // NO VALUE? -> DEFULT

    let threads: u16 = 4;                  // threads to add to the pool
    let max_requests: usize = usize::MAX;  // requests before automatic shutdown
    let listener_ip: &str = "127.0.255.1"; // send requests to this IP
    let listener_port: &str = "3467";      // Send Requests to this port (Rationale: DIMP typed on a telephone)

    let warn_restart_at: usize = (max_requests / 4) * 3; // this will round but won't error

    debug!("Launch Sequence Initated");

    // main portion
    debug!("Initalising Thread Pool");
    let thread_pool = ThreadPool::new(threads);

    // Anything that is intermediated by a server also has its contents
    // encrypted seperately, to avoid exposing information. All the server does
    // when one is involved is:
    // - [ ] Route Traffic
    // - [ ] Store ENCRYPTED data
    // - [ ] Cache information for zero-trust architecure.
    //      - [ ] This protocol, and this implementation, are built with the
    //        assumpstions that:
    //      - [ ] The only instance that can be trusted is that the user chose, and
    //        any other instance is unsafe and possibly malicuous.
    //      - [ ] Therefore, any possibly private data (IPs, Profile Names, Etc) is
    //        to be cached and proxied by the server.
    //
    // Since DIM is NOT A MEDIA TRANSFER PROTOCOL, it is common to store ata 
    // such as images eperately and transfer it via HTTPS, FTP, or a simillar
    // protocol. This is considered acceptable, as long as it maintains the same
    // zero-trust arcitecture, and can only be accessed by authorised parties.
    //
    // If a user attempts to access a media server or file they do not
    // explictly have access to, they are to get a https:403 Forbidden, reguardless
    // of the existance of media at that location.

    let identity = startup::get_identity("identity.pfx");
    let tcp_listener = TcpListener::bind("127.0.255.1:3467").unwrap();
    let tls_manager = startup::create_network_acceptor(identity);

    info!("Your server is running
{UL_ITEM}{BOLD}Software:{ENDBLOCK} {server_version}
{UL_ITEM}{BOLD}Threads:{ENDBLOCK} {threads}
{UL_ITEM}{BOLD}Max Requests:{ENDBLOCK} {max_requests} (Warn: 3/4ths)
{UL_ITEM}{BOLD}Location:{ENDBLOCK} {UNDERLINE}https://{listener_ip}:{listener_port}{ENDBLOCK}
{INDENT}If this is not correct, please press {BOLD}{UNDERLINE}CTRL+C{ENDBLOCK} during the 
{INDENT}launch countdown to abort the launch.");

    /* 📔 Note
     * set count to 0 to skip launch sequence. Is always 0 when hosting
     * a local server. Default is 5 secconds when testing anything else.
     * This mostly is to prevent the "onoseccond", where you do something
     * and immeditely realise that you have just made a very big mistake.
     */
    startup::launch_countdown(0);

    info!("Initation completed! Your server is now live! 🎉");

    for (packets_handled, stream) in tcp_listener
        .incoming()
        .enumerate()
        .take(max_requests)
    {
        let acceptor = tls_manager.clone();

        thread_pool.run(move || 
        {
            let stream = acceptor.accept(stream.unwrap());

            match &stream
            {
                Ok(_) => { },
                Err(_) => { return; }
            }

            connection_handler::handle_connection(stream
                .expect("Failed to read stream")
            );
        });

        if packets_handled == warn_restart_at
        {
            info!("Your server has passed the imment restart threadhold.
{INDENT}Your server will restart soon (ish). If this was too short, we
{INDENT}reccommend incresing your max packet limit, and restarting 
{INDENT}automatically at midnight in your timezone. A restart takes
{INDENT}less than three minutes, leaving minimal downtime if you
{INDENT}automate it.");
        }
        else if packets_handled == max_requests
        {
            info!("Shutting down: Max Packet Limit reached.");
            std::process::exit(255);
        }
        else if packets_handled >= warn_restart_at
        {
            info!("Please restart your server soon!");
        }

        trace!("lifetime: {packets_handled} packets handled");
    }

    info!("Begining server shutdown ...");
}