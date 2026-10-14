pub struct Object;
pub struct PyErr;
pub struct Python;
pub fn get_item(object: &Object, key: &Object, py: Python) -> Result<Object, PyErr> {
    let _ = (object, key, py);
    Ok(Object)
}
