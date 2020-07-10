use dict_derive::FromPyObject;

#[derive(FromPyObject)]
struct Derived<'a, 'b> {
    a: &'a str,
    b: &'b str,
}

fn main() {}
