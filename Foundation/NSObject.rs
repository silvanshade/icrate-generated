//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[objc2::protocol]
pub unsafe trait NSCoding {
    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "encodeWithCoder:")]
    unsafe fn encodeWithCoder(&self, coder: &NSCoder);

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Option<Id<Self>>;
}

#[objc2::protocol]
pub unsafe trait NSSecureCoding: NSCoding {
    #[objc2::method(sel = "supportsSecureCoding")]
    unsafe fn supportsSecureCoding() -> bool;
}

#[objc2::protocol]
pub unsafe trait NSDiscardableContent {
    #[objc2::method(sel = "beginContentAccess")]
    unsafe fn beginContentAccess(&self) -> bool;

    #[objc2::method(sel = "endContentAccess")]
    unsafe fn endContentAccess(&self);

    #[objc2::method(sel = "discardContentIfPossible")]
    unsafe fn discardContentIfPossible(&self);

    #[objc2::method(sel = "isContentDiscarded")]
    unsafe fn isContentDiscarded(&self) -> bool;
}

extern_fn!(
    pub unsafe fn NSAllocateObject(
        a_class: &Class,
        extra_bytes: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<Object>;
);

extern_fn!(
    pub unsafe fn NSDeallocateObject(object: &Object);
);

extern_fn!(
    #[deprecated = "Not supported"]
    pub unsafe fn NSCopyObject(
        object: &Object,
        extra_bytes: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<Object>;
);

extern_fn!(
    pub unsafe fn NSShouldRetainWithZone(an_object: &Object, requested_zone: *mut NSZone) -> Bool;
);

extern_fn!(
    pub unsafe fn NSIncrementExtraRefCount(object: &Object);
);

extern_fn!(
    pub unsafe fn NSDecrementExtraRefCountWasZero(object: &Object) -> Bool;
);

extern_fn!(
    pub unsafe fn NSExtraRefCount(object: &Object) -> NSUInteger;
);
