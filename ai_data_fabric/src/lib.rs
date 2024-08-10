use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use pyo3::exceptions::PyValueError;

#[pyclass]
#[derive(Clone)]
struct DataNode {
    data: Arc<Vec<f64>>,
    shape: Vec<usize>,
}

#[pymethods]
impl DataNode {
    #[new]
    fn new(data: Vec<f64>, shape: Vec<usize>) -> Self {
        DataNode {
            data: Arc::new(data),
            shape,
        }
    }

    #[getter]
    fn data(&self) -> Vec<f64> {
        self.data.to_vec()
    }

    fn transform(&self) -> Self {
        let results: Vec<f64> = self.data
            .par_iter()
            .map(|&x| x * 2.0)  // Simple transformation: doubling each value
            .collect();

        DataNode::new(results, self.shape.clone())
    }
}

#[pyclass]
struct AIDataFabric {
    data_sources: HashMap<String, DataNode>,
}

#[pymethods]
impl AIDataFabric {
    #[new]
    fn new() -> Self {
        AIDataFabric {
            data_sources: HashMap::new(),
        }
    }

    fn add_data_source(&mut self, name: String, data: Vec<f64>, shape: Vec<usize>) -> PyResult<()> {
        self.data_sources.insert(name, DataNode::new(data, shape));
        Ok(())
    }

    fn get_data_source(&self, name: &str) -> PyResult<DataNode> {
        self.data_sources.get(name)
            .cloned()
            .ok_or_else(|| PyValueError::new_err(format!("Data source '{}' not found", name)))
    }

    fn list_data_sources(&self) -> Vec<String> {
        self.data_sources.keys().cloned().collect()
    }

    fn remove_data_source(&mut self, name: &str) -> PyResult<()> {
        self.data_sources.remove(name)
            .ok_or_else(|| PyValueError::new_err(format!("Data source '{}' not found", name)))?;
        Ok(())
    }

    fn transform(&mut self, name: &str) -> PyResult<()> {
        let data_node = self.get_data_source(name)?;
        let transformed = data_node.transform();
        self.data_sources.insert(name.to_string(), transformed);
        Ok(())
    }

    fn __getitem__(&self, name: &str) -> PyResult<DataNode> {
        self.get_data_source(name)
    }

    fn __repr__(&self) -> String {
        format!("AIDataFabric(sources={})", self.list_data_sources().join(", "))
    }
}

#[pymodule]
fn ai_data_fabric(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DataNode>()?;
    m.add_class::<AIDataFabric>()?;
    Ok(())
}
