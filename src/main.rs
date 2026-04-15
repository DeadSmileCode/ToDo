use clap::{Arg, Command};
use anyhow::Ok;

use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use sqlx::{Row, QueryBuilder};
use std::str::FromStr;


//- Prepare struct for parse cli args
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





#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //- Parse cli args
    let matches = cli_args().get_matches();

    //- Open Sqlite pool
    let pool_opts = SqliteConnectOptions::from_str("sqlite://todos.sqlite")?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(pool_opts).await?;
    
    // connect("sqlite:./todos.sqlite").await?;

    //- If code run first time -> create table
    //- If exists -> do nothing
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            body TEXT
        )
        "#
    )
    .execute(&pool)
    .await?;

    //- Handle commands
    match matches.subcommand() {
        Some(("list", _)) => {
            let list = sqlx::query("SELECT id, title FROM todos")
                .fetch_all(&pool)
                .await?;

            for todo in list {
                let id: i32 = todo.get("id");
                let title: String = todo.get("title");
                let body: String = todo.get("title");

                println!("\x1b[1m{id}: {title}\x1b[0m\n\r\t{body}");
                println!("");
            }
        }
        Some(("add", sub_matches)) => {
            //- Prepare data from cli
            let title = sub_matches.get_many::<String>("title")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ")
                .to_string();

            let body= sub_matches.get_many::<String>("body")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(" ")
                .to_string();

            //- Printing for underdstand all good or not
            println!("Add new ToDo");
            println!("Title: {:?}", title);            
            println!("Body:");
            println!("{:?}", body);

            //- Insert data in to DB
            sqlx::query(
                r#"
                INSERT INTO todos (title, body) VALUES (?1, ?2)
                "#
            )
            .bind::<String>(title)
            .bind::<String>(body)
            .execute(&pool)
            .await?;
        }
        Some(("delete", sub_matches)) => {
            //- Prepare data from cli
            let list_ids: Vec<_> = sub_matches.get_many::<String>("list_id")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect();

            println!("Delete {:?}", list_ids);

            //- Delete ToDos
            let mut query = QueryBuilder::new("DELETE FROM todos WHERE id IN (");

            let mut separated = query.separated(", ");
            for id in list_ids {
                separated.push_bind(id);
            }            
            query.push(")");

            query.build()
                .execute(&pool)
                .await?;
        }
        _ => {
            println!("HAHAHA Imposible action!!");
        }
    }

    println!("Nothing to Do  XD XD XD");

    Ok(())
}