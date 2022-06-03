use std::io::{BufRead, Write};
use std::process::exit;
use clap::{Arg, ArgMatches, Command};
use typed_arena::Arena;
use crate::error::DResult;
use crate::operator::Operator;
use crate::splitter::Splitter;

pub struct StrSplit;

impl StrSplit {
    pub fn split_on_str<'a>(&self, inp: &mut dyn BufRead, out: &mut dyn Write, sep: &str) -> DResult<()> {
        for i in inp.lines() {
            match i {
                Ok(s) => {
                    for part in s.split(sep) {
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

    pub fn arg<'a>(&self) -> Arg<'a> {
        Arg::new("separator").required(true)
    }
}

impl Operator for StrSplit {
    fn command<'a>(&self, cmd: Command<'a>, _arena: &'a Arena<String>) -> Command<'a> {
        cmd
            .visible_alias("str")
            .visible_alias("char")
            .visible_alias("chr")
            .arg(self.arg())
    }

    fn name<'a>(&self, _arena: &'a Arena<String>) -> &'a str {
        "string"
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, args: &ArgMatches) -> DResult<()> {
        let sep = args.value_of("separator").expect("separator");
        self.split_on_str(inp, out, sep)
    }
}

impl Splitter for StrSplit {}
