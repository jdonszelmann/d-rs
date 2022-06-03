use crate::operator::Operator;

pub trait Splitter: Operator {

}

pub mod str_sep;
pub mod ws;

pub fn splitters() -> Vec<Box<dyn Splitter>> {
    vec![
        Box::new(str_sep::StrSplit),
        Box::new(ws::WsSplit)
    ]
}


