use log::{debug, error, info, trace, warn};
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

fn main()
{
    info!("Initalising Program");

    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    trace!("detailed tracing info");
    debug!("debug info");
    info!("relevant general info");
    warn!("warning you unwrap too much");
    error!("error message here");

    info!("This is the program speaking now!");

    // fatal!(901, "I had bad grammer and now I need to fix it");

    // Define an automatic-restart threashold. 
    let max_requests: usize = usize::MAX;

    verify_file_integrity();

    // Store these values. 
    // When someone logs on/connects and the server is running a version with
    // a known security vulnerability (manual override), or running a version
    // nearing the end of support, send a notification/system message
    // telling them. 
    check_updates();

    info!("Launch Sequence Initated");

    trace!("Initalising TCP Stream");

    let listner_ip = "127.0.0.1";
    let listner_port = "8800";

    info!("Listening to \x1b[4m{listner_ip}:{listner_port}\x1b[0m");

    warn!("{CODE_START}network_listner{ENDBLOCK} is bound to an UNWRAPPED VALUE!");
    let network_listener = TcpListener::bind(format!("{listner_ip}:{listner_port}")).unwrap();

    warn!("TCP Is NOT ENCRYPTED and NOT SPEC COMPLIANT! DIM protocol
             is actually built in TLS! This is just for testing!");

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

    let mut packets_handled: u128 = 0;
    let thread_pool = ThreadPool::new(4);


    warn!("When a payload is too lagre (over {} bytes), we simply 
{INDENT}drop the extra bytes rather than returning a 411 Payload_Too_Large!", u16::MAX);

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
        info!("Launching in \x1b[1m{n}{ENDBLOCK} secconds  \x1b[A\r", n=launch_countdown-count);
        thread::sleep(Duration::from_secs(1));
    }

    info!("Initation completed! Your server is now live! 🎉");

    // This automatically persists indefintely.
    for stream in network_listener.incoming().take(max_requests) // test shutdown
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

        packets_handled += 1;

        // This .unwrap() is 100% safe, since we check if its an `Err` type 
        // just above and if it is `continue;` the loop, skipping this block.
        thread_pool.run(|| { connection_handler::handle_connection(stream.unwrap()); });

        debug!("{packets_handled} packets handled");
    }

    info!("Begining server shutdown ...");
}

fn verify_file_integrity()
{
    // get repo from config files
    // get SHA-2 hash of files
    // Get desired SHA-2 hash from repo
    // compare them

    let local_software_hash = "Good2Go";
    let server_match_hash = "Good2Go";

    info!("Checking file integrity ...");
    if local_software_hash == server_match_hash
    {
        info!("File integrity good!");
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

fn check_updates()
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

    warn!("The function {CODE_START}check_updaes(){ENDBLOCK} currently has no functionality.");
    trace!("Checking for Updates");
}
