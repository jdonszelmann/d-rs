use std::io::{BufRead, Write};
use std::process::exit;
use clap::{Arg, ArgMatches, Command};
use typed_arena::Arena;
use crate::error::DResult;
use crate::operator::Operator;
use crate::reducer::Reducer;

pub struct Join;

impl Join {
    pub fn join(&self, inp: &mut dyn BufRead, out: &mut dyn Write, delim: &str) -> DResult<()> {
        let mut total = String::new();
        let mut pushed = false;

        for i in inp.lines() {
            match i {
                Ok(s) => {
                    if pushed {
                        total.push_str(delim);
                    }

                    total.push_str(&s);
                    pushed = true;
                }
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }
            }
        }

        writeln!(out, "{}", total)?;

        Ok(())
    }

    pub fn arg<'a>(&self) -> Arg<'a> {
        Arg::new("delimiter").default_value(" ")
    }
}

impl Operator for Join {
    fn command<'a>(&self, cmd: Command<'a>, _arena: &'a Arena<String>) -> Command<'a> {
        cmd
            .visible_alias("j")
            .arg(self.arg())
    }

    fn name<'a>(&self, _arena: &'a Arena<String>) -> &'a str {
        "join"
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, args: &ArgMatches) -> DResult<()> {
        let delim = args.value_of("delimiter").unwrap_or(" ");

        self.join(inp, out, delim)
    }
}

impl Reducer for Join {}
