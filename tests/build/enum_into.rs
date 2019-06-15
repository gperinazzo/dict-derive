use dict_derive::IntoPyObject;

#[derive(IntoPyObject)]
pub enum Nothing {
    None,
}

fn main() {}
