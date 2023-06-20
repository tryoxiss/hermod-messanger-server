macro_rules! log 
{
    ($message:expr) => 
    {
        eprintln!("\x1b[92m\x1b[1m         Log\x1b[0m {}", $message);
    }
}

macro_rules! info 
{
    ($message:expr) => 
    {
        eprintln!("\x1b[96m\x1b[1m        Info\x1b[0m {}", $message);
    }
}

// macro_rules! waiting 
// {
//     ($cycles:expr) => 
//     {
//         eprintln!("\x1b[96m\x1b[1m     Waiting\x1b[0m for input ({}) ...\x1b[F", $cycles)
//     }
// }

macro_rules! warning 
{
    ($message:expr) => 
    {
        eprintln!("\x1b[93m\x1b[1m     Warning\x1b[0m {}", $message);
    }
}

macro_rules! error 
{
    ($message:expr) => 
    {
        eprintln!("\x1b[91m\x1b[1m       Error\x1b[0m {}", $message);
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
        println!("\x1b[101m\x1b[30m\x1b[1m Pannicing! \x1b[0m\x1b[91m\x1b[1m Something happned that caused the program to panic");
        println!("\x1b[101m\x1b[30m\x1b[1m Message:   \x1b[0m\x1b[01m {}\x1b[0m {}", $code.to_string(), $message);
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m Why?       \x1b[0m This may have happened for one of three primary reasons:");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m The seccond digit in the error code should tell you which");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m category it fell under. 90X is entirely unknown.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m         1) \x1b[0m\x1b[91m\x1b[1m Modification Detected (91X)");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m The first, and most common reason, that this program pannics");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m is if a routinue check raises a concern. This could be anything");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m from a config file edited while running to a modified part of ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m the program binary.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m In this case, we panic to prevent things such as data corruption.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If this happned, please **shut down** your server before doing");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ANY sort of sysadmin.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m The seccond is much more sinster and means the program would have");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m blindly executed, likely malcious, injected code. On startup we");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m always check for a modified program binary, but if it is modified");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m during runtime we have no way to know except periodically checking.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m We check the SHA-2 hash of the programs binary every 15 minutes");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m by defult, and if we detect a modified binary we panic to prevent");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m any possible further damage.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m         2) \x1b[0m\x1b[91m\x1b[1m Set Up Wrong (92X)");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If the configuration is invalid, or the install is wrong, it may");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m realise it dosen't have all needed abilties during runtime, at");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m which point it will panic.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m This is generally due to a missing permission. We do our best to");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m check that the software has all needed capabilties before we open");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m the server, however sometimes things change at runtime. ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If you believe this happned due to a bug in the install script,");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m please report the bug.");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m         3) \x1b[0m\x1b[91m\x1b[1m A Program Bug (93X)");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If some code that is not completely safe made it into the relase");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m version you are running, we may have encoutered an `Err` or `None`");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m type that we didn't know how to handle. ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m If you believe this happned, please report this bug,");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m ");
        println!("\x1b[101m\x1b[30m\x1b[1m Report it! \x1b[0m\x1b[91m\x1b[1m If you believe 2 or 3 caused this, help us fix it!");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m Please report this as an issue, please raise an issue at:");
        println!("\x1b[101m\x1b[30m\x1b[1m            \x1b[0m\x1b[96m https://github.com/tryoxiss/bonfire-server/issues/new/choose");

        // 900 block: Emergency exit codes
        std::process::exit($code);
    }
}

#[cfg(test)]
mod tests
{
    #[test]
    fn test_alignment()
    {
        eprintln!("This is a VISUAL TEST!");
        eprintln!("Please make sure that the ends of the TITLE lines up evenly with all of the items.");
        
        eprintln!("\x1b[92m\x1b[1m       Title\x1b[0m The content message");

        log!("Log");
        info!("Info");
        warning!("Warning");
        error!("Error");
        fatal!("Fatal");
    }
}