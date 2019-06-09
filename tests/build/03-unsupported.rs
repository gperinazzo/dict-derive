use dict_derive::FromPyObject;

struct Test {
    test: String,
}

#[derive(FromPyObject)]
struct Derived {
    test: Test,
}

fn main() {}
