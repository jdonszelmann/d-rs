use std::io::Write;
use std::io::BufRead;
use clap::{ArgMatches, Command};
use typed_arena::Arena;
use crate::error::DResult;

pub trait Operator {
    fn command<'a>(&self, cmd: Command<'a>, arena: &'a Arena<String>) -> Command<'a>;
    fn name<'a>(&self, arena: &'a Arena<String>) -> &'a str;

    fn get_command<'a>(&self, arena: &'a Arena<String>) -> Command<'a> {
        self.command(Command::new(self.name(arena)), arena)
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, args: &ArgMatches) -> DResult<()>;
}