#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn gdal_data_path() {
        unsafe {
            let size = PDALGetGdalDataPath(ptr::null_mut(), 1024);
            assert_eq!(0, size);
        }
    }
}
