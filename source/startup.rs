use std::thread;

use log::trace;
use log::debug;
use log::info;
use log::warn;
use log::error;

use crate::INDENT;
use crate::BOLD;
use crate::ENDBLOCK;

pub fn launch_countdown(launch_countdown: u8) -> ()
{
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
}

pub fn verify_file_integrity(version: &String, repository: &str)
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

    warn!("The function `verify_file_integrity()` currently has 
{INDENT}no functionality.");
}

pub fn init_log4rs_config()
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

pub fn check_updates(current_major: u16, current_minor: u16, current_patch: u16, project: &str, release_level: &str, release_number: u16) -> String
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

    let mut release: String = String::from("");

    if release_level != "stable"
    {
        release = format!(":{release_level}.{release_number}");
    }

    warn!("The function `check_updaes()` currently has no functionality.");
    trace!("Checking for Updates");

    return String::from(format!("{project} {major}.{minor}.{patch}{release}",
            project = project.to_string(),
            major = current_major.to_string(),
            minor = current_minor.to_string(),
            patch = current_patch.to_string(),
            release = release
        )
    );
}

use std::sync::Arc;

use native_tls::TlsAcceptor;
pub fn create_network_acceptor(identity: Identity) -> Arc<TlsAcceptor>
{
    debug!("Initalize Network Listeners");
    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);

    return acceptor;
}

use native_tls::Identity;
use std::fs::File;
use std::io::Read;
use std::fs;
pub fn get_identity(file_path: &str) -> Identity
{
    debug!("Establishing Identity TLS Certificate.");
    let mut certificate_file = File::open(&file_path)
        .expect("`identity.pfx` could not be opened or was not found.");

    let mut certificate: Vec<u8> = vec![];

    certificate_file.read_to_end(&mut certificate).unwrap();

    // probably just read it from a SECRETS.ini or CERT_PASSWORD.pswd and not store it in RAM.
    let identity: Identity = Identity::from_pkcs12(&certificate, "admin").unwrap();

    return identity;
}

use std::io;

pub fn ask_yes_no(question: &str) -> bool
{
    let mut answer: String = String::from("");

    eprint!("\x1b[96m\x1b[1m         Ask\x1b[0m {} (Y/n) ", question);

    loop
    {
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read lines");

        answer = answer.trim().to_lowercase();
        trace!("Read answer as: {}", answer);

        if answer == String::from("y")
        {
            trace!("Read 'y' (yes), returning `true`.");
            return true;
        }
        else if answer == String::from("n")
        {
            trace!("Read 'n' (no), returning `false`.");
            return false;
        }
        else 
        {
            error!("Not a valid answer. Please input 'y' or 'n'.");
        }

        answer = String::from("");
    }
}