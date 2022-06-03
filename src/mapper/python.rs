use std::io::{BufRead, Write};
use std::process::{exit, Stdio};
use clap::{Arg, ArgMatches, Command};
use std::process;
use temp_file::{TempFile, with_contents};
use typed_arena::Arena;
use crate::error::DResult;
use crate::mapper::Mapper;
use crate::operator::Operator;

pub struct PythonMapper;

impl PythonMapper {
    pub fn map_to_python<'a>(&self, inp: &mut dyn BufRead, out: &mut dyn Write, program: &str) -> DResult<()> {
        let f: TempFile = with_contents(format!(r#"\
import sys
__oldprint = print
__printed = False
def print(*args, **kwargs):
    global __printed
    __printed = True
    __oldprint(*args, **kwargs)

inp = sys.stdin.readline()
i = inp
{program}

if "out" in globals() and not __printed:
    print(out)
if "o" in globals() and not __printed:
    print(o)
        "#).as_bytes());

        for i in inp.lines() {
            match i {
                Ok(s) => {
                    let mut proc = process::Command::new("python")
                        .arg(f.path())
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()?;

                    write!(proc.stdin.as_mut().expect("couldn't get stdin"), "{}", s)?;

                    let pout = proc.wait_with_output()?;
                    if !pout.status.success() {
                        eprintln!(r#"failed on line: "{}""#, s);
                        exit(1);
                    }
                    out.write_all(&pout.stdout)?;
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
        Arg::new("program").required(true)
            .multiple_values(true)
    }
}

impl Operator for PythonMapper {
    fn command<'a>(&self, cmd: Command<'a>, _arena: &'a Arena<String>) -> Command<'a> {
        cmd
            .visible_alias("p")
            .about("runs this provided program on every line. The `i` variable is the input. Output can be printed or written to `o`.")
            .arg(self.arg())
    }

    fn name<'a>(&self, _arena: &'a Arena<String>) -> &'a str {
        "python"
    }

    fn run(&self, inp: &mut dyn BufRead, out: &mut dyn Write, args: &ArgMatches) -> DResult<()> {
        let program = args.values_of("program").expect("program").collect::<Vec<_>>().join(" ");
        self.map_to_python(inp, out, &program)
    }
}

impl Mapper for PythonMapper {}
