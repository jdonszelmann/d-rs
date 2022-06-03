use std::io::{BufReader, stdin, stdout};
use clap::{Command, command};
use typed_arena::Arena;
use crate::error::DResult;
use crate::splitter::str_sep::StrSplit;
use crate::splitter::ws::WsSplit;

mod operator;

mod splitter;
mod mapper;
mod filterer;
mod reducer;
mod error;
mod regexes;

fn subcommands(arena: &Arena<String>) -> Vec<Command> {
   vec![
       Command::new("filter")
           .visible_alias("f")
           .arg_required_else_help(true)
           .subcommands(
               filterer::filterers()
                   .into_iter()
                   .map(|i| i.get_command(arena))
                   .collect::<Vec<_>>()
           ),

       Command::new("map")
           .visible_alias("m")
           .arg_required_else_help(true)
           .subcommands(
               mapper::mappers()
                   .into_iter()
                   .map(|i| i.get_command(arena))
                   .collect::<Vec<_>>()
           ),

       Command::new("split")
           .visible_alias("s")
           .arg(StrSplit.arg().required(false))
           .subcommands(splitter::splitters()
               .into_iter()
               .map(|i| i.get_command(arena))
               .collect::<Vec<_>>()
           ),

       Command::new("reduce")
           .visible_alias("r")
           .arg_required_else_help(true)
           .subcommands(reducer::reducers()
               .into_iter()
               .map(|i| i.get_command(arena))
               .collect::<Vec<_>>()
           ),
   ]
}

fn main() -> DResult<()> {
    let arena = Arena::new();

    let cmd: Command = command!();
    let cmd = cmd
        .arg_required_else_help(true)
        .subcommands(subcommands(&arena));

    let inp = &mut BufReader::new(stdin());
    let out = &mut stdout();

    let matches = cmd.get_matches();
    let subcommand = matches.subcommand().expect("parser should ensure only valid subcommand names are used");
    match subcommand {
        ("m" | "map", m) => {
            if let Some((name, args)) = m.subcommand() {
                if let Some(mapper) = mapper::mappers()
                    .into_iter()
                    .find(|i| {
                        i.name(&arena) == name
                    }) {
                    mapper.run(inp, out, args)?
                }
            } else {
                unreachable!("parser should ensure only valid subcommand names are used")
            }
        }
        ("r" | "reduce", m) => {
            if let Some((name, args)) = m.subcommand() {
                if let Some(mapper) = reducer::reducers()
                    .into_iter()
                    .find(|i| {
                        i.name(&arena) == name
                    }) {
                    mapper.run(inp, out, args)?
                }
            } else {
                unreachable!("parser should ensure only valid subcommand names are used")
            }
        }
        ("f" | "filter", m) => {
            if let Some((name, args)) = m.subcommand() {
                if let Some(mapper) = filterer::filterers()
                    .into_iter()
                    .find(|i| {
                        i.name(&arena) == name
                    }) {
                    mapper.run(inp, out, args)?
                }
            } else {
                unreachable!("parser should ensure only valid subcommand names are used")
            }
        }
        ("s" | "split", m) => {
            if let Some((name, args)) = m.subcommand() {
                if let Some(splitter) = splitter::splitters()
                    .into_iter()
                    .find(|i| {
                        i.name(&arena) == name
                    }) {
                    splitter.run(inp, out, args)?
                }
            } else if let Some(sep) = m.value_of("separator") {
                StrSplit.split_on_str(inp, out, sep)?
            } else {
                WsSplit.split_on_ws(inp, out)?
            }
        }
        _ => unreachable!("parser should ensure only valid subcommand names are used")
    }

    Ok(())
}
