mod python;
mod regex;

use std::ops::Deref;
use crate::mapper::python::PythonMapper;
use crate::mapper::regex::{RegexMapper, SpecificRegexMapper};
use crate::operator::Operator;
use crate::regexes::REGEXES;

pub trait Mapper: Operator {

}

pub fn mappers() -> Vec<Box<dyn Mapper>> {
    let mut res: Vec<Box<dyn Mapper>> = vec![
        Box::new(PythonMapper),
        Box::new(RegexMapper),
    ];

    for i in REGEXES.deref().clone().into_iter() {
        res.push(Box::new(SpecificRegexMapper::from(i)));
    }

    res

}
