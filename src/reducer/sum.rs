use std::io::{BufRead, Write};
use std::process::exit;
use clap::{ArgMatches, Command};
use typed_arena::Arena;
use crate::error::DResult;
use crate::operator::Operator;
use crate::reducer::Reducer;

pub struct Sum;

impl Sum {
    pub fn sum<'a>(&self, inp: &mut dyn BufRead, out: &mut dyn Write) -> DResult<()> {
        let mut total = 0.0;

        for i in inp.lines() {
            match i {
                Ok(s) => {
                    total += s.parse::<f64>()
                        .map_err(|e| format!("failed to parse {s} as float: {e}"))?;
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
}

impl Operator for Sum {
    fn command<'a>(&self, cmd: Command<'a>, _arena: &'a Arena<String>) -> Command<'a> {
        cmd
            .visible_alias("s")
            .visible_alias("+")
    }

    fn name<'a>(&self, _arena: &'a Arena<String>) -> &'a str {
        "sum"
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, _args: &ArgMatches) -> DResult<()> {
        self.sum(inp, out)
    }
}

impl Reducer for Sum {}
