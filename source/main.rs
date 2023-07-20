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

// Make our own logging system, with messaghes being *what is it doing* and not *what level is it*
#[macro_use]
mod terminal_out;
use terminal_out::ask_yes_no;

mod threading;
use threading::ThreadPool;

use std::net::{TcpListener, TcpStream};
use native_tls::{Identity, TlsAcceptor, TlsStream};

use std::thread;

use std::fs;
use std::fs::File;
use std::io::{Read};
use std::sync::Arc;

use log::{trace, debug, info, warn, error};
use log4rs;

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
    init_log4rs_config();

    info!("Initalising Program");

    const REPOSITORY: &str = "tryoxiss.github.io";
    let server_version: String = check_updates();
    verify_file_integrity(&server_version, REPOSITORY);

    // config variables

    // READ CONFIG FILE
    // ASSIGN VALUES
    // NO VALUE? -> DEFULT

    // these will need to be let eventually
    const THREADS: u16 = 4;                  // threads to add to the pool
    const MAX_REQUESTS: usize = usize::MAX;  // requests before automatic shutdown
    const LISTENER_IP: &str = "127.0.255.1"; // send requests to this IP
    const LISTENER_PORT: &str = "3467";      // Send Requests to this port (Rationale: DIMP typed on a telephone)

    const WARN_RESTART_AT: usize = (MAX_REQUESTS / 4) * 3; // this will round but won't error

    // variable_trace!("THREADS", THREADS);

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
    let thread_pool = ThreadPool::new(THREADS);

    // debug!("Initalising TCP Stream");

    // // 🚩 FIXME: TCP is not encrypted!
    // // We need this TCP stream, but all content needs to be send over a secure
    // // end-to-end-encrypted channel. All items recieved must be decrypted, all
    // // items sent must be encrypted. This encryption will happen with
    // // AES-GCM-SIV.
    // //
    // // We also need to perform the AES/TLS/SSL handshakes to make this work.
    // // Anything that is intermediated by a server also has its contents
    // // encrypted seperately, to avoid exposing information. All the server does
    // // when one is involved is:
    // // - Route Traffic
    // // - Store ENCRYPTED data
    // // - Cache information for zero-trust architecure.
    // //      - This protocol, and this implementation, are built with the
    // //        assumpstions that:
    // //      - The only instance that can be trusted is that the user chose, and
    // //        any other instance is unsafe and possibly malicuous.
    // //      - Therefore, any possibly private data (IPs, Profile Names, Etc) is
    // //        to be cached and proxied by the server.
    // //
    // // Since DIM is NOT A MEDIA TRANSFER PROTOCOL, it is not uncommon to store
    // // data seperately and transfer it via HTTPS, FTP, or a simillar protocol.
    // // This is considered acceptable, as long as it maintains the same
    // // zero-trust arcitecture, and can only be accessed by authorised parties.
    // //
    // // If a user attempts to access a media server or file they do not
    // // explictly have access to, they are to get a 403 Forbidden, reguardless
    // // of the existance of media at that location.
//     let network_listener = TcpListener::bind(format!("{LISTENER_IP}:{LISTENER_PORT}"))
//         .expect(format!("Failed to bind to listner address {}:{}", LISTENER_IP, LISTENER_PORT).as_str());

    let mut file = File::open("identity.pfx").unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();
    let identity = Identity::from_pkcs12(&identity, "admin").unwrap();

    let network_listener = TcpListener::bind("127.0.255.1:3467").unwrap();
    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);

    info!("Your server is running
{UL_ITEM}{BOLD}Software:{ENDBLOCK} {server_version}
{UL_ITEM}{BOLD}Threads:{ENDBLOCK} {THREADS}
{UL_ITEM}{BOLD}Max Requests:{ENDBLOCK} {MAX_REQUESTS} (Warn: 3/4ths)
{UL_ITEM}{BOLD}Location:{ENDBLOCK} {UNDERLINE}https://{LISTENER_IP}:{LISTENER_PORT}{ENDBLOCK}
{INDENT}If this is not correct, please press {BOLD}{UNDERLINE}CTRL+C{ENDBLOCK} during the 
{INDENT}launch countdown to abort the launch.");

    /* 📔 Note
     * set count to 0 to skip launch sequence. Is always 0 when hosting
     * a local server. Defult is 5 secconds when testing anything else.
     * This mostly is to prevent the "onoseccond", where you do something
     * and immeditely realise that you have just made a very big mistake.
     */
    let launch_countdown: u8 = 0;

    for count in 0..launch_countdown
    {
        use std::time::Duration;
        /* 📔 Note
         * The extra spaces get rid of trailing "s" characters when the digits drop.
         * e.g.
         * Launching in 10 secconds
         * launching in 9 seccondss
         *                        ^ Stayed because it was never overwritten.
         * We only allow up to a count of 256 (u8), so two trailing spaces is enough.
         */
        info!("Launching in {BOLD}{n}{ENDBLOCK} secconds  \x1b[A\r", n=launch_countdown-count);
        thread::sleep(Duration::from_secs(1));
    }

    info!("Initation completed! Your server is now live! 🎉");

    warn!("We currently are encrypting with {CODE_START}OsRng{ENDBLOCK}, which is NOT 
{INDENT}(NECESARLY) CRYPTOGRAPHICALLY SECURE!!
{INDENT}It often is, but not always!");



    for (packets_handled, stream) in network_listener
        .incoming()
        .enumerate()
        .take(MAX_REQUESTS)
    {
        match &stream {
            Ok(stream) => { info!("TCP stream found!"); }
            Err(error) => 
            { 
                error!("TCP Stream is an {CODE_START}Err{ENDBLOCK} type!! Something went
{INDENT}terribly wrong! Attached is the compilers error
{error}"); 
                continue;
            }
        }

        let acceptor = acceptor.clone();
        
        // This .unwrap() is 100% safe, since we check if its an `Err` type 
        // just above and if it is `continue;` the loop, skipping this block.
        thread_pool.run(move || 
        { 
            info!("Meow 2");

            let stream = acceptor.accept(stream.unwrap());

            match &stream
            {
                Ok(T) => { info!("Shit do be good right meow"); },
                Err(E) => { warn!("Hissy fit"); }
            }

            info!("Meow");

            // stream
            //     .expect("wth?")
            //     .write("Hiss");

            connection_handler::handle_connection(stream.unwrap()); 
        });

        if packets_handled == WARN_RESTART_AT
        {
            info!("Your server has passed the imment restart threadhold.
{INDENT}Your server will restart soon (ish). If this was too short, we
{INDENT}reccommend incresing your max packet limit, and restarting 
{INDENT}automatically at midnight in your timezone. A restart takes
{INDENT}less than three minutes, leaving minimal downtime if you
{INDENT}automate it.");
        }
        else if packets_handled == MAX_REQUESTS
        {
            info!("\x1b[01mYour server is now shutting down{ENDBLOCK}, since you reached your
{INDENT}maximum packet limit. We will send a notification to you and your 
{INDENT}servers usersers, informaing them of what happened.");

            // This is not needed to shut down the program, but it is useful
            // as the code is passed to the operating system, allowing for
            // automation of tasks such as spinning up a new server to replace
            // this one.
            std::process::exit(255);
        }
        else if packets_handled >= WARN_RESTART_AT
        {
            info!("Plesae restart your server soon! It has handled more
{INDENT}than 3/4ths of its single-run lifetime.
{INDENT}({packets_handled} / {MAX_REQUESTS} packets handled)");
        }

        trace!("lifetime: {packets_handled} packets handled");
    }

    info!("Begining server shutdown ...");
}

// SETUP HELPER FUNCTIONS

fn verify_file_integrity(version: &String, repository: &str)
{
    // 🚧 TODO: verify_file_integrity()
    // get repo from config files
    // get SHA-2 hash of files
    // Get desired SHA-2 hash from repo
    //      - Likely something like $REPO/hashes/VERSION_STRING.txt
    // compare them

    // if repo does not exist
    // {
    //    info!("We could not find the repository {CODE_START}{repository}{ENDBLOCK}.");
    // }

    // if repo/versions/VERSION_STRING.hash does not exist
    // {
    //    info!("Could not find <Repo>/.checksums/{version}.hash in the repositories file tree.");
    // }

    // replace with actual hashes
    let local_software_hash: &str = "Good2Go";
    let server_match_hash: &str = "Good2Go";

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

        // 🚧 TODO: Make it actually fix files
        // Re-install the software if needed.
        if fix_files 
        {
            info!("Fixing Files <NOT IMPLEMENTED YET>");
        }
    }

    warn!("The function {CODE_START}verify_file_integrity(){ENDBLOCK} currently has 
{INDENT}no functionality.");
}

fn init_log4rs_config()
{
    log4rs::init_file("log4rs.yml", Default::default())
        .expect("Failed to init log4rs file!");
}

fn create_log4rs_file()
{
    let mut _file = File::create("log4rs.yml");
    match fs::write("log4rs.yml", 
b"\
appenders:
    stdout:
    # TODO: 
    # - Make `Capitalsed` instead of `UPPERCASE`.
    # - Change Colors
    #   - Trace: Grey
    #   - Debug: Green
    #   - Info:  Blue
    #   - Warn:  Yellow (Already Correct)
    #   - Error: Red    (Already Correct)
        kind: console
        encoder:
            pattern: \"{h(\\x1b[1m{l}):>16.16}\\x1b[0m {m}{n}\"
    file:
        kind: file
        path: \"logs/recent.log\"
        encoder:
            pattern: \"{d(%Y-%m-%d %H:%M:%S)} : {m}{n}\"
root:
    level: trace
    appenders:
        - stdout
        - file")
    {
        Ok(_) =>
        {
            println!("The defult log4rs.yml file has been created! Try re-running the program!");
            std::process::exit(1);
        }

        Err(error) =>
        {
            panic!("No log4rs.yml file existed and it was failed to be created. Here is the error
{error}");
        }
    }
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
