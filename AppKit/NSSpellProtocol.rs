//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSChangeSpelling {
        #[method(changeSpelling:)]
        unsafe fn changeSpelling(&self, sender: Option<&Object>);
    }

    unsafe impl ProtocolType for dyn NSChangeSpelling {}
);

extern_protocol!(
    pub unsafe trait NSIgnoreMisspelledWords {
        #[method(ignoreSpelling:)]
        unsafe fn ignoreSpelling(&self, sender: Option<&Object>);
    }

    unsafe impl ProtocolType for dyn NSIgnoreMisspelledWords {}
);
