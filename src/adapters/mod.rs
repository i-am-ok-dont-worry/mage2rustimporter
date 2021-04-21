pub mod product;
pub mod category;
pub mod attributes;

pub trait ImportAdapter {
    fn run(&mut self, ids: Option<Vec<&str>>);
}

