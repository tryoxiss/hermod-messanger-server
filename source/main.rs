/*
This file is a part of Hermod Messanger Server.

	Copyright (C) 2023-Present Hermod Messanger Contributers. (AUTHORS.md)
	Under The GNU Affero General Public Licence 3.0 ONLY (LICENCE.md)

	If for any reason the licence file was not provided, you may obtain a
	copy at <https://www.gnu.org/licenses/agpl-3.0.txt>.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.
*/

/*
This file was authored by:
	* Madeline "tryoxiss"
	* Khaim "khaim0919"
*/

// We currently have no need for this, and likely never will!
// #![forbid(unsafe_code)]

mod connection;
mod startup;
//mod connection::packets;

mod threading;
use threading::ThreadPool;

// Make our own logging system, with messaghes being *what is it doing* and not *what level is it*
use log::trace;
use log::debug;
use log::info;
// use log::warn;
// use log::error;

const CODE_START: &str = "\x1b[40m";
const ENDBLOCK: &str   = "\x1b[0m";
const INDENT: &str	 = "             ";
const BOLD: &str	   = "\x1b[1m";
const UNDERLINE: &str  = "\x1b[4m";

//      Regular Indent -------------
const UL_ITEM: &str	= "              •";

fn main() -> ()
{
	// 🚩 FIXME: The main log file needs to be renammed to when it was run
	// once a new file is created/after the program ends, so that recent.log
	// can take its place instead of appending new logs to recent.log
	startup::init_log4rs_config();
	// TODO: Use SimplestLogger, once I get it more done and get it on crates.io
	// (or even if I don't, its great and especially for this!)

	info!("Initalising program master thread");

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

	let threads: u16 = 4;
	let max_requests: usize = usize::MAX;
	let listener_ip:[u16; 8] = [0, 0, 0, 0, 0, 0, 0, 1]; // send requests to this IP
	let listener_port: u16 = 3467; // Send Requests to this port (Rationale: DIMP typed on a telephone)

	let warn_restart_at: usize = (max_requests / 4) * 3; // this will round but won't error

	// main portion
	debug!("Initalising Thread Pool");
	let thread_pool = ThreadPool::new(threads);

	// Anything that is intermediated by a server also has its contents
	// encrypted seperately, to avoid exposing information. All the server does
	// when one is involved is:
	// - [ ] Route Traffic
	// - [ ] Store ENCRYPTED data
	// - [ ] Cache information for zero-trust architecure.
	//	  - [ ] This protocol, and this implementation, are built with the
	//		assumpstions that:
	//	  - [ ] The only instance that can be trusted is that the user chose, and
	//		any other instance is unsafe and possibly malicuous.
	//	  - [ ] Therefore, any possibly private data (IPs, Profile Names, Etc) is
	//		to be cached and proxied by the server.
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
	// let tcp_listener = startup::tcp_bind("127.0.255.1", "3467");
	let tcp_listener = startup::tcp_bind(listener_ip, listener_port);
	let tls_manager = startup::create_network_acceptor(identity);

	// Instead of constant terminal messages maybe have this one box which prints important info constantly?


	// can be configured, but is info box by default

	// Box drawing characters are 2 characters wide, so if these look like they
	// are not lined up your IDE is displaying them as thin, not correct.
//	 let production_message: String = format!("\
// ╭──────────────────────────────╮
// │ [Server Name]											  │
// ├──────────────────────────────┤
// │ Uptime: 0h 0m 0s										   │
// │ Memory: 00KiB											  │
// │ Threads: 1/4											   │
// │ Request Buffer: 0										  │
// │ Requests: 0 / {max_requests}						 │
// │															│
// │ hermod_server 0.3.5:pre-release.4						  │
// ╰──────────────────────────────╯
// ").to_string();

	// terminal_out::status_box();

//	 println!("{}", production_message);

//	 let production_message: String = format!("\
// ╭──────────────────────────────╮
// │ [Server Name]											  │
// ├──────────────────────────────┤
// │ Uptime: 0h 0m 0s										   │
// │ Memory: 00KiB											  │
// │ Threads: 1/4											   │
// │ Request Buffer: 0										  │
// │ Requests: 0 / {max_requests:>20}						 │
// │															│
// │ hermod_server 0.3.5:pre-release.4						  │
// ╰──────────────────────────────╯
// ").to_string();

//	 println!("{}", production_message);


	let formatted_listner_ip = format!(
		"{a}:{b}:{c}:{d}:{e}:{f}:{g}:{h}",
		a = listener_ip[0],
		b = listener_ip[1],
		c = listener_ip[2],
		d = listener_ip[3],
		e = listener_ip[4],
		f = listener_ip[5],
		g = listener_ip[6],
		h = listener_ip[7],
	);

	info!("Your server is running
{UL_ITEM}{BOLD}Software:{ENDBLOCK} {server_version}
{UL_ITEM}{BOLD}Threads:{ENDBLOCK} {threads}
{UL_ITEM}{BOLD}Max Requests:{ENDBLOCK} {max_requests} (Warn: 3/4ths)
{UL_ITEM}{BOLD}Location:{ENDBLOCK} {UNDERLINE}https://[{formatted_listner_ip}]:{listener_port}{ENDBLOCK}
{INDENT}If this is not correct, please press {BOLD}{UNDERLINE}CTRL+C{ENDBLOCK} during the 
{INDENT}launch countdown to abort the launch.");

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

			connection::handle(stream.expect("Failed to read stream"));
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

	info!("Shutdown in progress");
	// info!("Begining server shutdown ...");
}