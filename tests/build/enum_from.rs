use dict_derive::FromPyObject;

#[derive(FromPyObject)]
pub enum Nothing {
    None,
}

fn main() {}
