pub struct Reader { pub version: u32 }
/// The version of the export data format, read from its header.
pub fn version(r: &Reader) -> u32 {
    r.version
}
