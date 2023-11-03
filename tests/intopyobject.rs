extern crate pyo3;
use dict_derive::IntoPyObject;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[derive(IntoPyObject, Debug)]
struct User {
    name: Option<String>,
    email: String,
    age: u16,
}

#[derive(IntoPyObject, Debug)]
struct UserWithLifetime<'a> {
    name: Option<&'a str>,
    email: &'a str,
    age: u16,
}

#[test]
fn test_conversion() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        {
            let user = User {
                name: None,
                email: "tester@tests.com".to_owned(),
                age: 27,
            };
            let obj: PyObject = user.into_py(py);
            let dict: &PyDict = py.checked_cast_as(obj)?;

            assert!(dict.get_item("name").unwrap().is_some());
            assert!(dict.get_item("email").unwrap().is_some());
            assert!(dict.get_item("age").unwrap().is_some());

            let name: Option<&str> = dict.get_item("name").unwrap().unwrap().extract()?;
            assert_eq!(name, None);

            let email: &str = dict.get_item("email").unwrap().unwrap().extract()?;
            assert_eq!(email, "tester@tests.com");

            let age: u16 = dict.get_item("age").unwrap().unwrap().extract()?;
            assert_eq!(age, 27);
        }

        {
            let user = User {
                name: Some("Test".to_owned()),
                email: "tester@tests.com".to_owned(),
                age: 27,
            };
            let obj: PyObject = user.into_py(py);
            let dict: &PyDict = py.checked_cast_as(obj)?;

            assert!(dict.get_item("name").unwrap().is_some());

            let name: Option<&str> = dict.get_item("name").unwrap().unwrap().extract()?;
            assert_eq!(name, Some("Test"));
        }
        {
            let user = UserWithLifetime {
                name: Some("Test"),
                email: "tester@tests.com",
                age: 27,
            };
            let obj: PyObject = user.into_py(py);
            let dict: &PyDict = py.checked_cast_as(obj)?;

            assert!(dict.get_item("name").unwrap().is_some());
            assert!(dict.get_item("email").unwrap().is_some());

            let name: Option<&str> = dict.get_item("name").unwrap().unwrap().extract()?;
            assert_eq!(name, Some("Test"));

            let email: &str = dict.get_item("email").unwrap().unwrap().extract()?;
            assert_eq!(email, "tester@tests.com");
        }
        Ok(())
    })
}
