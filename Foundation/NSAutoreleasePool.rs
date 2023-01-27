//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSAutoreleasePool")]
    pub struct NSAutoreleasePool;

    #[cfg(feature = "Foundation_NSAutoreleasePool")]
    unsafe impl ClassType for NSAutoreleasePool {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSAutoreleasePool")]
unsafe impl NSObjectProtocol for NSAutoreleasePool {}

extern_methods!(
    #[cfg(feature = "Foundation_NSAutoreleasePool")]
    unsafe impl NSAutoreleasePool {
        #[method(addObject:)]
        pub unsafe fn addObject_class(an_object: &Object);

        #[method(addObject:)]
        pub unsafe fn addObject(&self, an_object: &Object);

        #[method(drain)]
        pub unsafe fn drain(&self);
    }
);
