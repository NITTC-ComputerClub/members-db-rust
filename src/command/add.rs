use clap::Clap;

use crate::command::ICommand;
use crate::database::member::Member;
use crate::database::Database;
use crate::Opts;

#[derive(Clap)]
pub struct AddCommand {
    name: String,

    #[clap(long)]
    office365: Option<String>,
}

impl ICommand for AddCommand {
    fn execute(&self, opts: &Opts) {
        let path = String::from(&opts.database);
        let mut database = Database::new(path);
        let member = Member::new(self.name.clone())
            .with_office365(self.office365.clone());

        database.add_member(member);
        database.save();
    }
}
