use std::{
    env::{self, Args},
    process,
};

use mini_grep::{run, ConfigBuiler};

fn main() {
    let mut args: Args = env::args();
    args.next();

    let config = ConfigBuiler::new()
        .query(args.next())
        .file_path(args.next())
        .ignore_case(args.any(|arg| arg == "--i" || arg == "--ignore-case"))
        .build()
        .unwrap_or_else(|error| {
            println!("parsing arguments failed: {error}");
            process::exit(1);
        });

    if let Err(error) = run(config) {
        println!("operation failed: {}", error);
        process::exit(1);
    }
}
