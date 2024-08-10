# Create project directory
mkdir ai_data_fabric
cd ai_data_fabric

# Initialize Rust project
cargo init

# Set up Python binding
cargo add pyo3
cargo add pyo3-macros

# Create Python module directory
mkdir python_module

# Create initial Python file
touch python_module/__init__.py

# Create initial Rust file
echo "use pyo3::prelude::*;

#[pymodule]
fn ai_data_fabric(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AIDataFabric>()?;
    Ok(())
}

#[pyclass]
struct AIDataFabric {
    // Add fields here
}

#[pymethods]
impl AIDataFabric {
    #[new]
    fn new() -> Self {
        AIDataFabric {}
    }

    // Add methods here
}
" > src/lib.rs

# Initialize git repository
git init
echo "/target
/Cargo.lock
__pycache__
*.pyc
*.pyo
*.pyd
*.so
*.dylib
" > .gitignore

git add .
git commit -m "Initial project setup"