use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Example function with error handling
#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyValueError::new_err("Division by zero is not allowed"));
    }
    Ok(a / b)
}

/// Example function that accepts various Python types
#[pyfunction]
fn process_list(items: Vec<String>) -> PyResult<usize> {
    Ok(items.len())
}

/// Example Python class implemented in Rust
#[pyclass]
struct {{crate_name|pascal_case}} {
    value: i32,
}

#[pymethods]
impl {{crate_name|pascal_case}} {
    /// Create a new instance
    #[new]
    fn new(value: i32) -> Self {
        {{crate_name|pascal_case}} { value }
    }

    /// Get the current value
    fn get_value(&self) -> i32 {
        self.value
    }

    /// Set a new value
    fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    /// Increment the value
    fn increment(&mut self, amount: i32) -> i32 {
        self.value += amount;
        self.value
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("{{crate_name|pascal_case}}(value={})", self.value)
    }

    /// String representation for str()
    fn __str__(&self) -> String {
        format!("{{crate_name|pascal_case}} with value: {}", self.value)
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn {{crate_name}}(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add functions
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    m.add_function(wrap_pyfunction!(process_list, m)?)?;
    
    // Add classes
    m.add_class::<{{crate_name|pascal_case}}>()?;
    
    // Add module-level constants or variables
    m.add("__version__", "0.1.0")?;
    
    Ok(())
}
