use bayard_client::client::client::{create_client, Clerk};
use clap::ArgMatches;

use crate::util::log::set_logger;

pub fn run_schema_cli(matches: &ArgMatches) -> Result<(), String> {
    set_logger();

    let servers: Vec<_> = matches
        .values_of("SERVERS")
        .unwrap()
        .map(|addr| create_client(addr))
        .collect();

    let client_id = rand::random();

    let mut client = Clerk::new(&servers, client_id);
    let value = client.schema();
    print!("{}", value);

    Ok(())
}
