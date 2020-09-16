# Dict-Derive

Derive macro for PyO3's `FromPyObject` and `IntoPy<PyObject>` traits. The derived implementation turns a Python's dict into your Rust struct and back.

## Usage

Add the library to your `Cargo.toml` together with PyO3:
```
[dependencies]
pyo3 = "0.9"
dict_derive = "0.2.0"
```

Import the derive implementation and use it on your structs:
```rust
extern crate dict_derive;

use dict_derive::{FromPyObject, IntoPyObject};


#[derive(FromPyObject, IntoPyObject)]
struct User {
    name: String,
    email: String,
    age: u32,
}
```

Then you can use your structs as arguments and return values in your PyO3 functions:
```rust
extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Requires FromPyObject to receive a struct as an argument
#[pyfunction]
fn get_contact_info(user: User) -> PyResult<String> {
    Ok(format!("{} - {}", user.name, user.email))
}

// Requires IntoPyObject to return a struct
#[pyfunction]
fn get_default_user() -> PyResult<User> {
    Ok(User {
        name: "Default".to_owned(),
        email: "default@user.com".to_owned(),
        age: 27,
    })
}
```

And then call your functions using normal python dicts:
```
>>> import mylib
>>> mylib.get_contact_info({"name": "Thor", "email": "thor@asgard.com", "age": 23})
'Thor - thor@asgard.com'
>>> mylib.get_default_user()
{'name': 'Default', 'email': 'default@user.com', 'age': 27}
```


### Older PyO3 versions
- For PyO3 version 0.7.0 or lower, use version 0.1 of this crate.
- For PyO3 version 0.8 to 0.10, use version 0.2 of this crate.
