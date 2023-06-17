pub struct NetworkConnection
{
    // iobuffer: Vec<char>,
    prop: String,
}

// trait NetworkConnection
// {
//     fn establish_connection();
//     fn update_connection();
//     fn terminate_connection();
// }

impl NetworkConnection
{
    pub fn establish_connection() -> NetworkConnection
    {
        warning!("The function `get_connection()` currently has no functionality.");
        log!("Getting Connection");

        return NetworkConnection { prop: String::from("PlaceholderData") }
    }
}