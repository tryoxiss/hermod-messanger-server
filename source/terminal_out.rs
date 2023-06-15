macro_rules! log 
{
    ($message:expr) => 
    {
        eprintln!("\x1b[37m\x1b[92m         Log\x1b[0m {}", $message);
    }
}

macro_rules! info 
{
    ($message:expr) => 
    {
        eprintln!("\x1b[96m\x1b[1m        Info\x1b[0m {}", $message);
    }
}

macro_rules! waiting 
{
    ($cycles:expr) => 
    {
        eprintln!("\x1b[96m\x1b[97m     Waiting\x1b[0m for input ({}) ...\x1b[F", $cycles)
    }
}

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
    ($message:expr) => 
    {
        eprintln!("       \x1b[101m\x1b[30m\x1b[1mFatal\x1b[0m {}", $message);
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