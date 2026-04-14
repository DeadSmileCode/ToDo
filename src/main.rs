use clap::{Arg, Command};

fn cli_args() -> Command {
    Command::new("my-todo")
        .about("First my ToDo app")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("list")
                .about("Show all ToDo plans"),
        )
        .subcommand(
            Command::new("add")
                .about("Add new ToDo")
                .arg(
                    Arg::new("title")
                    .short('t')
                    .num_args(0..)
                    .help("Short title for ToDo")
                )
                .arg(
                    Arg::new("body")
                    .long("body")
                    .short('b')
                    .num_args(0..)
                    .help("Main description with full details")
                )
        )
        .subcommand(
            Command::new("delete")
                .about("Delete task")
                .arg(
                    Arg::new("list_id")
                    .num_args(0..)
                    .help("IDs of ToDo wich you need to delete")
                )
                .arg_required_else_help(true),
        )
}


fn main() {
    let matches = cli_args().get_matches();

    match matches.subcommand() {
        Some(("list", _)) => {
            println!("List");
        }
        Some(("add", sub_matches)) => {
            println!("Add new ToDo");

            let title: Vec<_> = sub_matches.get_many::<String>("title")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect();

            println!("Title: {:?}", title.join(" "));

            let body: Vec<_> = sub_matches.get_many::<String>("body")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect();

            println!("Body:");
            println!("{:?}", body.join(" "));
        }
        Some(("delete", sub_matches)) => {
            let list_ids: Vec<_> = sub_matches.get_many::<String>("list_id")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect();

            println!("Delete {:?}", list_ids);
        }
        _ => {
            println!("HAHAHA Imposible action!!");
        }
    }

    println!("Nothing to Do  XD XD XD");
}