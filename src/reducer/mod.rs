use crate::operator::Operator;

pub trait Reducer: Operator {

}

mod sum;
mod prod;
mod join;

pub fn reducers() -> Vec<Box<dyn Reducer>> {
    vec![
        Box::new(sum::Sum),
        Box::new(prod::Prod),
        Box::new(join::Join),
    ]
}


