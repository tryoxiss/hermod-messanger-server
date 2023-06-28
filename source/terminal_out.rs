// macro_rules! trace
// {
//     ($message:expr) =>
//     {
//         eprintln!("\x1b[92m\x1b[1m         Log\x1b[0m {}", $message);
//     }
// }

// macro_rules! info
// {
//     ($message:expr) =>
//     {
//         eprintln!("\x1b[96m\x1b[1m        Info\x1b[0m {}", $message);
//     }
// }

// macro_rules! waiting
// {
//     ($cycles:expr) =>
//     {
//         eprintln!("\x1b[96m\x1b[1m     Waiting\x1b[0m for input ({}) ...\x1b[F", $cycles)
//     }
// }

// macro_rules! warn
// {
//     ($message:expr) =>
//     {
//         eprintln!("\x1b[93m\x1b[1m     Warning\x1b[0m {}", $message);
//     }
// }

// macro_rules! error
// {
//     ($message:expr) =>
//     {
//         eprintln!("\x1b[91m\x1b[1m       Error\x1b[0m {}", $message);
//     }
// }

use std::io;

use log::{trace, debug, info, warn, error};

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

macro_rules! fatal
{
    // Codes:
    //
    // 910 = unknown modification
    // 911 = Program Binary Modified
    // 912 = Config File Modified During Runtime
    // 913 = Hardware Modified During Runtime
    //
    // 920 = Unknown Invalid Setup
    // 921 = Config Invalid
    // 922 = Config Contradctary
    // 923 = Server Power Out
    //
    // 930 = Unknown Program Bug
    // 931 = Data Injection Error
    // 932 = Main Thread Paniced
    ($code:expr, $message:expr) =>
    {
        println!("\x1b[101m\x1b[30m\x1b[1m Panicking! \x1b[0m\x1b[91m\x1b[1m Something happened that caused the program to panic");
        println!("\x1b[101m\x1b[30m\x1b[1m   Message: \x1b[0m\x1b[01m {}\x1b[0m {}", $code.to_string(), $message);
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m Why?       \x1b[0m This may have happened for one of three primary reasons.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m The second digit tells you which one it likely was, 92X is likely");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m #2, etc.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m         1) \x1b[0m\x1b[91m\x1b[1m Modification Detected (91X)");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m The first, and most common situation, that causes this program to");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m panic is when a routine check raises a concern. This could be");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m anything from a config file edited while running to a modified ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m part of the program binary.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m In this case, we panic to prevent things such as data corruption.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If this happened, please **shut down** your server before doing");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ANY sort of sysadmin.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m This may also be more sinister and means the program would have");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m blindly executed, likely malicious, injected code. On startup we");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m always check for a modified program binary, but if it is modified");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m during runtime we have no way to know except periodically checking.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m We check the SHA-2 hash of the programs binary every 15 minutes");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m by default, and if we detect a modified binary we panic to prevent");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m any possible further damage.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m         2) \x1b[0m\x1b[91m\x1b[1m Set Up Wrong (92X)");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If the configuration is invalid, or the install is wrong, it may");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m realize it does not have all needed abilities during runtime, at");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m which point it will panic. This is almost always due to a");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m permission being changed while the program is running.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m This is generally due to a missing permission. We do our best to");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m check that the software has all needed capabilities before we open");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m the server, however sometimes things change at runtime. ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If you believe this happened due to a bug in the install script,");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m please report the bug.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m         3) \x1b[0m\x1b[91m\x1b[1m A Program Bug (93X)");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If some code that is not completely safe made it into the release");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m version you are running, we may have encountered an `Err` or `None`");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m type that we didn't know how to handle. ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If you believe this happened, please report this bug,");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m Report it! \x1b[0m\x1b[91m\x1b[1m If you believe 2 or 3 caused this, help us fix it!");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m Please report this as an issue, please raise an issue at:");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m\x1b[96m https://github.com/tryoxiss/bonfire-server/issues/new/choose");

        // 900 block: Emergency exit codes
        std::process::exit($code);
    }
}