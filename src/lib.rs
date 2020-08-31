#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn pdal_version() {
        let mut version_raw = [0; 42];
        let len = unsafe { PDALVersionString(version_raw.as_mut_ptr(), 42) };
        let version = unsafe {
            CStr::from_ptr(version_raw.as_ptr())
                .to_string_lossy()
                .into_owned()
        };
        assert_eq!(len as usize, version.len());
        let version_major = unsafe { PDALVersionMajor() };
        let version_minor = unsafe { PDALVersionMinor() };
        let version_patch = unsafe { PDALVersionPatch() };
        assert_eq!(
            format!("{}.{}.{}", version_major, version_minor, version_patch),
            version
        )
    }
}
