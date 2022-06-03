use std::io::{BufRead, Write};
use std::process::exit;
use clap::{ArgMatches, Command};
use typed_arena::Arena;
use crate::error::DResult;
use crate::operator::Operator;
use crate::splitter::Splitter;

pub struct WsSplit;

impl WsSplit {
    pub fn split_on_ws<'a>(&self, inp: &mut dyn BufRead, out: &mut dyn Write) -> DResult<()> {
        for i in inp.lines() {
            match i {
                Ok(s) => {
                    for part in s.split_whitespace() {
                        writeln!(out, "{}", part)?;
                    }
                }
                Err(e) => {
                    eprintln!("{e}");
                    exit(1);
                }
            }
        }
        Ok(())
    }
}

impl Operator for WsSplit {
    fn command<'a>(&self, cmd: Command<'a>, _arena: &'a Arena<String>) -> Command<'a> {
        cmd
            .visible_alias("ws")
    }

    fn name<'a>(&self, _arena: &'a Arena<String>) -> &'a str {
        "whitespace"
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, _args: &ArgMatches) -> DResult<()> {
        self.split_on_ws(inp, out)
    }
}

impl Splitter for WsSplit {}
