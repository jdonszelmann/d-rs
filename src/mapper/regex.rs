use std::io::{BufRead, Write};
use std::process::exit;
use clap::{Arg, ArgMatches, Command};
use fancy_regex::Regex;
use typed_arena::Arena;
use crate::error::DResult;
use crate::mapper::Mapper;
use crate::operator::Operator;


pub struct SpecificRegexMapper(Regex, String, Vec<String>);

impl From<(String, (Regex, Vec<String>))> for SpecificRegexMapper {
    fn from((name, (regex, aliases)): (String, (Regex, Vec<String>))) -> Self {
        SpecificRegexMapper(regex, name, aliases)
    }
}

impl SpecificRegexMapper {
    pub fn args<'a>(&self) -> Vec<Arg<'a>> {
        vec![
            Arg::new("single")
                .takes_value(false)
                .short('s')
                .required(false)
                .help("only map the first match"),

            Arg::new("find")
                .takes_value(false)
                .short('f')
                .long("find")
                .required(false)
                .help("only map if found"),

            Arg::new("replace")
                .default_value("$0"),
        ]
    }
}


impl Operator for SpecificRegexMapper {
    fn command<'a>(&self, cmd: Command<'a>, arena: &'a Arena<String>) -> Command<'a> {
        let mut cmd = cmd.args(self.args());

        for i in &self.2 {
            let s = arena.alloc(i.clone()).as_str();
            cmd = cmd.visible_alias(s)
        }

        cmd = cmd.about(arena.alloc(format!(
            "like using: /{}/",
            self.0.as_str().chars().take(15).collect::<String>()
        )).as_str());

        cmd
    }

    fn name<'a>(&self, arena: &'a Arena<String>) -> &'a str {
        arena.alloc(self.1.clone()).as_str()
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, args: &ArgMatches) -> DResult<()> {
        let single = args.is_present("single");
        let find = args.is_present("find");
        let replace = args.value_of("replace").unwrap_or("$0");

        RegexMapper.map_regex(inp, out, self.0.clone(), replace, single, find)
    }
}

impl Mapper for SpecificRegexMapper {}

pub struct RegexMapper;

impl RegexMapper {
    pub fn map_regex<'a>(&self, inp: &mut dyn BufRead, out: &mut dyn Write, regex: Regex, replace: &str, single: bool, find: bool) -> DResult<()> {
        for i in inp.lines() {
            match i {
                Ok(s) => {
                    if !find || (find && regex.find(&s)?.is_some()) {
                        writeln!(out, "{}", if single {
                            regex.replace_all(&s, replace)
                        } else {
                            regex.replace(&s, replace)
                        })?;
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

    pub fn map<'a>(&self, inp: &mut dyn BufRead, out: &mut dyn Write, regex: &str, replace: &str, single: bool, find: bool) -> DResult<()> {
        let regex = Regex::new(regex)?;
        self.map_regex(inp, out, regex, replace, single, find)
    }

    pub fn args<'a>(&self) -> Vec<Arg<'a>> {
        vec![
            Arg::new("regex")
            .required(true),
            Arg::new("replace")
                .default_value("$0"),
            Arg::new("single")
                .takes_value(false)
                .short('s')
                .required(false)
                .help("only map the first match"),
            Arg::new("find")
                .takes_value(false)
                .short('f')
                .long("find")
                .required(false)
                .help("only map if found"),
        ]
    }
}

impl Operator for RegexMapper {
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
        let replace = args.value_of("replace").unwrap_or("$0");
        let single = args.is_present("single");
        let find = args.is_present("find");

        self.map(inp, out, regex, replace, single, find)
    }
}

impl Mapper for RegexMapper {}
