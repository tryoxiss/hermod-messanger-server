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


use log::{trace, debug, info, warn, error};
use log4rs;

// #[macro_use]
mod terminal_out;
use terminal_out::ask_yes_no;

mod threading;
use threading::ThreadPool;

use std::net::TcpListener;

use std::time::Duration;
use std::thread;

mod connection_handler;

static CODE_START: &str = "\x1b[40m";
static ENDBLOCK: &str   = "\x1b[0m";
static INDENT: &str     = "             ";
static BOLD: &str       = "\x1b[1m";

fn main()
{
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("Initalising Program");

    // config variables

    // READ CONFIG FILE
    // ASSIGN VALUES
    // NO VALUE? -> DEFULT

    let threads: usize = 4;               // threads to add to the pool
    let max_requests: usize = usize::MAX; // requests before automatic shutdown
    let listner_ip: &str = "127.0.255.1"; // send requests to this IP
    let listner_port: &str = "8800";      // Send Requests to this port

    // when do we send a warning the server is reaching its capacity?
    let warn_restart = (max_requests / 4) * 3;

    verify_file_integrity();

    // Store these values. 
    // When someone logs on/connects and the server is running a version with
    // a known security vulnerability (manual override), or running a version
    // nearing the end of support, send a notification/system message
    // telling them. 
    let server_version: String = check_updates();

    debug!("Launch Sequence Initated");

    // ACTUALLY it seems Aes-Gcm-Siv handles all this for us!
    // ---
    // We can use a CSPRNG or Crypographically Secure Psudo-Random Number
    // Generator for encryption values. We will use ChaCha20-poly1305 since
    // - It can produce 1.8gb of randomness every seccond, making it far from
    //   a bottleneck.
    // - initalises fast (but startup times are not very important)
    // - only uses 136 bytes of perpetual memory 
    // - has been [deeply analyised](ChaCha20Analysis) 
    // 
    // [ChaCha20Analysis]: https://datatracker.ietf.org/doc/html/rfc7539#section-1
    // (Same as Above)   : https://www.cryptrec.go.jp/exreport/cryptrec-ex-2601-2016.pdf
    // (Summary)         : https://en.wikipedia.org/wiki/ChaCha20-Poly1305
    // ---

    // main portion
    debug!("Initalising Thread Pool");
    let thread_pool = ThreadPool::new(threads);

    warn!("When a payload is too lagre (over {} bytes), we simply 
{INDENT}drop the extra bytes rather than returning a 411 Payload_Too_Large!", u16::MAX);

    debug!("Initalising TCP Stream");

    warn!("TCP Is NOT ENCRYPTED and NOT SPEC COMPLIANT! DIM protocol
            is actually built in TLS! This is just for testing!");
    let network_listener = TcpListener::bind(format!("{listner_ip}:{listner_port}"));

    match network_listener
    {
        Ok(_) =>
        {
            debug!("Network listeer initalised!");
        }

        Err(error) =>
        {
            error!("An error occured when binding to the network listner.
    {INDENT}Here is the rust compiler message:
    {error}");
            panic!();
        }
    }

    info!("Your server is running
{INDENT} -  {BOLD}Software:{ENDBLOCK} {server_version}
{INDENT} -  {BOLD}Threads:{ENDBLOCK} {threads}
{INDENT} -  {BOLD}Max Requests:{ENDBLOCK} {max_requests} (warn at 3/4ths though)
{INDENT} -  {BOLD}Location:{ENDBLOCK} \x1b[4mhttp://{listner_ip}:{listner_port}{ENDBLOCK}
{INDENT}If this is not correct, please press CTRL+C during the 
{INDENT}launch countdown to abort the launch.");

    // set count to 0 to skip launch sequence
    let launch_countdown: u8 = 0;

    for count in 0..launch_countdown
    {
        // The extra spaces get rid of trailing "s" characters when the digits drop.
        // e.g.
        // Launching in 10 secconds
        // launching in 9 seccondss
        //                        ^ Stayed because it was never overwritten.
        // We only allow up to a count of 256 (u8), so two trailing spaces is enough.
        info!("Launching in {BOLD}{n}{ENDBLOCK} secconds  \x1b[A\r", n=launch_countdown-count);
        thread::sleep(Duration::from_secs(1));
    }

    info!("Initation completed! Your server is now live! 🎉");

    warn!("We currently are encrypting with {CODE_START}OsRng{ENDBLOCK}, which is NOT 
{INDENT}(NECESARLY) CRYPTOGRAPHICALLY SECURE!!
{INDENT}It often is, but not always!");

    // This automatically persists indefintely.
    // This unwrap is 100% since since we get out of the way earlier if its an Err
    for (packets_handled, stream) in network_listener
        .unwrap()
        .incoming()
        .enumerate()
        .take(max_requests)
    {
        match &stream 
        {
            Ok(message) => { trace!("Stream is OK"); }

            Err(error) =>
            {
                error!("TCP Stream is an {CODE_START}Err{ENDBLOCK} type!! Something went
{INDENT}terribly wrong! Attached is the compilers error
{error}");
                continue;
            }
        }

        // This .unwrap() is 100% safe, since we check if its an `Err` type 
        // just above and if it is `continue;` the loop, skipping this block.
        thread_pool.run(|| { connection_handler::handle_connection(stream.unwrap()); });

        if packets_handled == warn_restart
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
            info!("\x1b[01mYour server is now shutting down{ENDBLOCK}, since you reached your
{INDENT}maximum packet limit. We will send a notification to you and your 
{INDENT}servers usersers, informaing them of what happened.");
        }
        else if packets_handled >= warn_restart
        {
            info!("Plesae restart your server soon! It has handled more
{INDENT}than 3/4ths of its single-run lifetime.
{INDENT}({packets_handled} / {max_requests} packets handled)");
        }

        trace!("lifetime: {packets_handled} packets handled");

        // println!("\x1b[30mEnter Command ...\x1b[A\r{ENDBLOCK}");
    }

    info!("Begining server shutdown ...");
}

fn verify_file_integrity()
{
    // get repo from config files
    // get SHA-2 hash of files
    // Get desired SHA-2 hash from repo
    // compare them

    // replace with actual hashes
    let local_software_hash = "Good2Go";
    let server_match_hash = "Good2Go";

    trace!("Checking file integrity ...");
    if local_software_hash == server_match_hash
    {
        debug!("File integrity good!");
    }
    else
    {
        error!("File hashes do not match. If you just set this server up, 
{INDENT}check that the repo listed in your bonfire.config` file is 
{INDENT}the same as where you got your source code from. (If you 
{INDENT}used an install script, it's most likely correct)
{INDENT}
{INDENT}if it is not a fresh install, this likely means your files 
{INDENT}are either corrupted or tampered with!");

        let fix_files = ask_yes_no("Would you like to fix your files?");
        // todo!("Implement this!");

        if fix_files 
        {
            info!("Fixing Files <NOT IMPLEMENTED YET>");
        }
    }

    warn!("The function {CODE_START}verify_file_integrity(){ENDBLOCK} currently has 
{INDENT}no functionality.");
    trace!("Veryfying file integrity");
}

fn check_updates() -> String
{
    // get repo from config file
    // get version from config file
    // Get desired hash from repo
    // get version from repo
    // compare version
    // if version is less than (patch, A.A.THIS): 
    //      Suggest Update
    // if version is less than (secrity A.A.THIS AND A.A.CURRENT_VERSION is DEPRECATED or YANKED):
    //      *strongly* suggest update
    // if version is less than (major): 
    //      suggest update when next doing major admin stuff/setting up new servers

    let project: &str = "hermod";
    let major: u16 = 0;
    let minor: u16 = 3;
    let patch: u16 = 5;

    let release_level: &str = "pre-release";
    let release_number: u16 = 4;

    let mut release: String = String::from("");

    if release_level != "stable"
    {

        release = format!(":{release_level}.{release_number}");
    }


    warn!("The function {CODE_START}check_updaes(){ENDBLOCK} currently has no functionality.");
    trace!("Checking for Updates");

    return String::from(format!("{project} {major}.{minor}.{patch}{release}",
            project = project.to_string(),
            major = major.to_string(),
            minor = minor.to_string(),
            patch = patch.to_string(),
            release = release
        )
    );
}
