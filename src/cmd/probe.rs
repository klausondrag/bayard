use bayard_client::client::client::{create_client, Clerk};
use clap::ArgMatches;

use crate::util::log::set_logger;

pub fn run_probe_cli(matches: &ArgMatches) -> Result<(), String> {
    set_logger();

    let server = matches.value_of("SERVER").unwrap();

    let mut servers = Vec::new();
    servers.push(create_client(server));

    let client_id = rand::random();

    let mut client = Clerk::new(&servers, client_id);
    let value = client.probe();
    print!("{}", value);

    Ok(())
}
