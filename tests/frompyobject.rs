extern crate pyo3;
use dict_derive::FromPyObject;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::PyErrValue;

#[derive(FromPyObject, Debug)]
struct User {
    name: Option<String>,
    email: String,
    age: u16,
}

fn make_user_dict<'a, A, B, C>(
    py: &'a Python,
    email: A,
    age: B,
    name: Option<Option<C>>,
) -> PyResult<&'a PyDict>
where
    A: ToPyObject,
    B: ToPyObject,
    C: ToPyObject,
{
    let dict = PyDict::new(*py);

    if let Some(opt) = name {
        dict.set_item("name", opt)?;
    }

    dict.set_item("email", email)?;
    dict.set_item("age", age)?;
    Ok(dict)
}

#[test]
fn test_conversion() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    {
        let dict = make_user_dict(&py, "tester@tests.com", 27, Some(Some("Tester")))?;

        let result: PyResult<User> = dict.extract();

        assert!(result.is_ok());
        let user = result.unwrap();

        assert_eq!(&user.name.unwrap(), "Tester");
        assert_eq!(&user.email, "tester@tests.com");
        assert_eq!(user.age, 27);
    }

    {
        let name: Option<Option<&str>> = None;
        let dict = make_user_dict(&py, "tester@tests.com", 27, name)?;

        let result: PyResult<User> = dict.extract();

        assert!(result.is_ok());
        let user = result.unwrap();

        assert_eq!(user.name, None);
        assert_eq!(&user.email, "tester@tests.com");
        assert_eq!(user.age, 27);
    }
    {
        let name: Option<Option<&str>> = Some(None);
        let dict = make_user_dict(&py, "tester@tests.com", 27, name)?;

        let result: PyResult<User> = dict.extract();

        assert!(result.is_ok());
        let user = result.unwrap();

        assert_eq!(user.name, None);
        assert_eq!(&user.email, "tester@tests.com");
        assert_eq!(user.age, 27);
    }
    Ok(())
}

#[test]
fn test_type_error() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = make_user_dict(&py, "tester@tests.com", "27", Some(Some("Test")))?;

    let result: PyResult<User> = dict.extract();
    assert!(result.is_err());
    let err = result.unwrap_err();

    assert!(err.is_instance::<pyo3::exceptions::TypeError>(py));

    if let PyErrValue::ToObject(obj) = err.pvalue {
        let value: PyObject = (*obj).to_object(py);
        let result: String = value.extract(py)?;
        assert_eq!(&result, "Unable to convert key: age");
    } else {
        panic!("Not To Object");
    }

    Ok(())
}

#[test]
fn test_missing_key() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = make_user_dict(&py, "tester@tests.com", 27, Some(Some("Test")))?;
    dict.del_item("age")?;

    let result: PyResult<User> = dict.extract();
    assert!(result.is_err());
    let err = result.unwrap_err();

    assert!(err.is_instance::<pyo3::exceptions::ValueError>(py));

    if let PyErrValue::ToObject(obj) = err.pvalue {
        let value: PyObject = (*obj).to_object(py);
        let result: String = value.extract(py)?;
        assert_eq!(&result, "Missing required key: age");
    } else {
        panic!("Not To Object");
    }
    Ok(())
}

#[test]
fn test_wrong_type() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let list = PyList::new(py, vec![1, 2, 3]);

    let result: PyResult<User> = list.extract();
    assert!(result.is_err());
    let err = result.unwrap_err();

    assert!(err.is_instance::<pyo3::exceptions::TypeError>(py));

    if let PyErrValue::ToObject(obj) = err.pvalue {
        let value: PyObject = (*obj).to_object(py);
        let result: String = value.extract(py)?;
        assert_eq!(&result, "Invalid type to convert, expected dict");
    } else {
        panic!("Not To Object");
    }
    Ok(())
}

use std::option;

#[derive(FromPyObject)]
struct TotallyOptionalUser {
    name: Option<String>,
    email: option::Option<String>,
    age: std::option::Option<String>,
    address: core::option::Option<String>,
}

#[test]
fn test_optionals() -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let dict = PyDict::new(py);
    let result: PyResult<TotallyOptionalUser> = dict.extract();

    assert!(result.is_ok());
    let user = result.unwrap();

    assert_eq!(user.name, None);
    assert_eq!(user.email, None);
    assert_eq!(user.age, None);
    assert_eq!(user.address, None);

    Ok(())
}
