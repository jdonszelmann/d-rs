mod regex;

use std::ops::Deref;
use crate::filterer::regex::SpecificRegexFilter;
use crate::operator::Operator;
use crate::regexes::REGEXES;

pub trait Filterer: Operator {}

pub fn filterers() -> Vec<Box<dyn Filterer>> {
    let mut res: Vec<Box<dyn Filterer>> = vec![
        Box::new(regex::RegexFilter),
    ];

    for i in REGEXES.deref().clone().into_iter() {
        res.push(Box::new(SpecificRegexFilter::from(i)));
    }

    res
}
