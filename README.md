# Dict-Derive

Derive macro for PyO3's `FromPyObject` trait. The derived implementation turns a Python's dict into your Rust struct.

## Usage

Add the library to your `Cargo.toml` together with PyO3:
```
[dependencies]
pyo3 = "0.7.0"
dict_derive = "0.1.0"
```

Import the derive implementation and use it on your structs:
```rust
extern crate dict_derive;

use dict_derive::FromPyObject;


#[derive(FromPyObject)]
struct User {
    name: String,
    email: String,
    age: u32,
}
```

Then you can use your structs as arguments in your PyO3 functions:
```rust
extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn get_contact_info(user: User) -> PyResult<String> {
    Ok(format!("{} - {}", user.name, user.email))
}
```

And then call your functions using normal python dicts:
```
>>> import mylib
>>> mylib.get_contact_info({"name": "Thor", "email": "thor@asgard.com", "age": 23})
'Thor - thor@asgard.com'
```
