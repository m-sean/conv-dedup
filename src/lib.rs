mod dedup;
mod lsh;

use pyo3::prelude::*;

#[pymodule]
fn lsh_dedup(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<dedup::DeduplicationTable>()?;
    m.add_class::<lsh::MinHash>()?;
    m.add_class::<lsh::MinHashLSH>()?;
    m.add_class::<lsh::Record>()?;
    Ok(())
}
