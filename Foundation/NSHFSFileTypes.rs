//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSFileTypeForHFSTypeCode(hfs_file_type_code: OSType) -> *mut NSString;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSHFSTypeCodeFromFileType(file_type_string: Option<&NSString>) -> OSType;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSHFSTypeOfFile(full_file_path: Option<&NSString>) -> *mut NSString;
);
