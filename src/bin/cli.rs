use clap::{arg, Arg, Command};
use cr8s::commands::{create_user, delete_user, list_users};

extern crate cr8s;

fn main() {
    let matches = Command::new("Cr8s")
        .version("1.0")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User administrative commands")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new user")
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(
                            Arg::new("roles")
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(Command::new("list").about("List existing users"))
                .subcommand(
                    Command::new("delete").about("Delete an existing user").arg(
                        Arg::new("id")
                            .required(true)
                            .value_parser(clap::value_parser!(i32)),
                    ),
                ),
        )
        .subcommand(
            Command::new("digest-send")
                .about("Send an email with the newest crates")
                .arg(Arg::new("to").required(true))
                .arg(
                    Arg::new("hours_since")
                        .required(true)
                        .value_parser(clap::value_parser!(i32)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => create_user(
                sub_matches
                    .get_one::<String>("username")
                    .unwrap()
                    .to_owned(),
                sub_matches
                    .get_one::<String>("password")
                    .unwrap()
                    .to_owned(),
                sub_matches
                    .get_many::<String>("roles")
                    .unwrap()
                    .map(|v| v.to_string())
                    .collect(),
            ),
            Some(("list", _)) => list_users(),
            Some(("delete", sub_matches)) => {
                delete_user(sub_matches.get_one::<i32>("id").unwrap().to_owned())
            }
            _ => {}
        },
        Some(("digest-send", sub_matches)) => cr8s::commands::send_digest(
            sub_matches.get_one::<String>("to").unwrap().to_owned(),
            sub_matches
                .get_one::<i32>("hours_since")
                .unwrap()
                .to_owned(),
        ),
        _ => {}
    }
}
