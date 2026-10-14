pub struct Object;
pub struct PyErr;
pub fn get_item(object: &Object, key: &Object) -> Result<Object, PyErr> {
    let _ = (object, key);
    Ok(Object)
}
