use std::io::{BufRead, Write};
use std::process::exit;
use clap::{ArgMatches, Command};
use typed_arena::Arena;
use crate::error::DResult;
use crate::operator::Operator;
use crate::reducer::Reducer;

pub struct Prod;

impl Prod {
    pub fn product<'a>(&self, inp: &mut dyn BufRead, out: &mut dyn Write) -> DResult<()> {
        let mut total = 1.0;

        for i in inp.lines() {
            match i {
                Ok(s) => {
                    total *= s.parse::<f64>()
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

impl Operator for Prod {
    fn command<'a>(&self, cmd: Command<'a>, _arena: &'a Arena<String>) -> Command<'a> {
        cmd
            .visible_alias("prod")
            .visible_alias("p")
            .visible_alias("*")
    }

    fn name<'a>(&self, _arena: &'a Arena<String>) -> &'a str {
        "product"
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, _args: &ArgMatches) -> DResult<()> {
        self.product(inp, out)
    }
}

impl Reducer for Prod {}
