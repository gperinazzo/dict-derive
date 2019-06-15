use dict_derive::IntoPyObject;

struct Test {
    test: String,
}

#[derive(IntoPyObject)]
struct Derived {
    test: Test,
}

fn main() {}
