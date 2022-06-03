use std::io::{BufRead, Write};
use std::process::exit;
use clap::{Arg, ArgMatches, Command};
use fancy_regex::Regex;
use typed_arena::Arena;
use crate::error::DResult;
use crate::filterer::Filterer;
use crate::operator::Operator;

pub struct SpecificRegexFilter(Regex, String, Vec<String>);

impl From<(String, (Regex, Vec<String>))> for SpecificRegexFilter {
    fn from((name, (regex, aliases)): (String, (Regex, Vec<String>))) -> Self {
        SpecificRegexFilter(regex, name, aliases)
    }
}

impl SpecificRegexFilter {
    pub fn arg<'a>(&self) -> Arg<'a> {
        Arg::new("single")
            .takes_value(false)
            .short('s')
            .required(false)
            .help("only map the first match")
    }
}


impl Operator for SpecificRegexFilter {
    fn command<'a>(&self, cmd: Command<'a>, arena: &'a Arena<String>) -> Command<'a> {
        let mut cmd = cmd.arg(self.arg());

        for i in &self.2 {
            let s = arena.alloc(i.clone()).as_str();
            cmd = cmd.visible_alias(s)
        }

        cmd
    }

    fn name<'a>(&self, arena: &'a Arena<String>) -> &'a str {
        arena.alloc(self.1.clone()).as_str()
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, args: &ArgMatches) -> DResult<()> {
        let single = args.is_present("single");

        RegexFilter.map_regex(inp, out, self.0.clone(), single)
    }
}

impl Filterer for SpecificRegexFilter {}

pub struct RegexFilter;

impl RegexFilter {
    pub fn map_regex<'a>(&self, inp: &mut dyn BufRead, out: &mut dyn Write, regex: Regex, single: bool) -> DResult<()> {
        for i in inp.lines() {
            match i {
                Ok(s) => {
                    if single {
                        for i in regex.find_iter(&s) {
                            writeln!(out, "{}", i?.as_str())?;
                        }
                    } else {
                        if let Some(f) = regex.find(&s)? {
                            writeln!(out, "{}", f.as_str())?;
                        }
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

    pub fn map<'a>(&self, inp: &mut dyn BufRead, out: &mut dyn Write, regex: &str, single: bool) -> DResult<()> {
        let regex = Regex::new(regex)?;

        self.map_regex(inp, out, regex, single)
    }

    pub fn args<'a>(&self) -> Vec<Arg<'a>> {
        vec![
            Arg::new("regex")
            .required(true),
            Arg::new("single")
                .takes_value(false)
                .short('s')
                .required(false)
                .help("only map the first match")
        ]
    }
}

impl Operator for RegexFilter {
    fn command<'a>(&self, cmd: Command<'a>, _arena: &'a Arena<String>) -> Command<'a> {
        cmd
            .visible_alias("r")
            .args(self.args())
    }

    fn name<'a>(&self, _arena: &'a Arena<String>) -> &'a str {
        "regex"
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, args: &ArgMatches) -> DResult<()> {
        let regex = args.value_of("regex").expect("separator");
        let single = args.is_present("single");

        self.map(inp, out, regex, single)
    }
}

impl Filterer for RegexFilter {}
