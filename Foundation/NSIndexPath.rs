//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSIndexPath")]
    pub struct NSIndexPath;

    #[cfg(feature = "Foundation_NSIndexPath")]
    unsafe impl ClassType for NSIndexPath {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSIndexPath")]
    unsafe impl NSIndexPath {
        #[method_id(@__retain_semantics Other indexPathWithIndex:)]
        pub unsafe fn indexPathWithIndex(index: NSUInteger) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other indexPathWithIndexes:length:)]
        pub unsafe fn indexPathWithIndexes_length(
            indexes: *mut NSUInteger,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithIndexes:length:)]
        pub unsafe fn initWithIndexes_length(
            this: Option<Allocated<Self>>,
            indexes: *mut NSUInteger,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithIndex:)]
        pub unsafe fn initWithIndex(
            this: Option<Allocated<Self>>,
            index: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other indexPathByAddingIndex:)]
        pub unsafe fn indexPathByAddingIndex(&self, index: NSUInteger) -> Id<NSIndexPath, Shared>;

        #[method_id(@__retain_semantics Other indexPathByRemovingLastIndex)]
        pub unsafe fn indexPathByRemovingLastIndex(&self) -> Id<NSIndexPath, Shared>;

        #[method(indexAtPosition:)]
        pub unsafe fn indexAtPosition(&self, position: NSUInteger) -> NSUInteger;

        #[method(length)]
        pub unsafe fn length(&self) -> NSUInteger;

        #[method(getIndexes:range:)]
        pub unsafe fn getIndexes_range(
            &self,
            indexes: NonNull<NSUInteger>,
            position_range: NSRange,
        );

        #[method(compare:)]
        pub unsafe fn compare(&self, other_object: &NSIndexPath) -> NSComparisonResult;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSIndexPath")]
    unsafe impl NSIndexPath {
        #[deprecated]
        #[method(getIndexes:)]
        pub unsafe fn getIndexes(&self, indexes: NonNull<NSUInteger>);
    }
);
