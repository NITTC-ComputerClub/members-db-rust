use clap::Clap;

use std::fs::create_dir_all;
use std::path::PathBuf;

use crate::command::ICommand;
use crate::database::Database;
use crate::{filesystem, Opts};

#[derive(Clap)]
pub struct BuildCommand {
    #[clap(short, long, default_value = "public")]
    directory: String,

    #[clap(short, long, default_value = "members.json")]
    filename: String,
}

impl ICommand for BuildCommand {
    fn execute(&self, opts: &Opts) {
        let path = String::from(&opts.database);
        let database = Database::new(path);
        let output: PathBuf = [&self.directory, &self.filename].iter().collect();

        create_dir_all(&self.directory).expect("Failed to create directories.");

        filesystem::save(
            output.to_str().expect("Failed to resolve path."),
            serde_json::to_string(database.get_members()).expect("Failed to serialize."),
        )
    }
}
