use markdown_rust_core::chunker;
use markdown_rust_core::lexer;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyDict};

#[pyfunction]
pub fn chunk_markdown(py: Python<'_>,text: &str) -> PyResult<Py<PyAny>> {
    let tokens = lexer::lex(text);
    let chunks = chunker::chunk(&tokens);

    let list = PyList::empty(py);
    for chunk in chunks {
        let dict = PyDict::new(py);
        dict.set_item("heading", chunk.heading)?;
        dict.set_item("text", chunk.text)?;
        dict.set_item("start_line", chunk.start_line)?;
        list.append(dict)?;
    }

    Ok(list.into_any().unbind())
}

#[pymodule]
fn markdown_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(chunk_markdown, m)?)?;
    Ok(())
}
