//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

#[ns_options]
#[underlying(NSUInteger)]
pub enum MTLFunctionOptions {
    MTLFunctionOptionNone = 0,
    MTLFunctionOptionCompileToBinary = 1 << 0,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLFunctionDescriptor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MTLFunctionDescriptor;
}

#[cfg(feature = "Metal_MTLFunctionDescriptor")]
unsafe impl NSObjectProtocol for MTLFunctionDescriptor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLFunctionDescriptor")]
    pub type MTLFunctionDescriptor;

    #[objc2::method(sel = "functionDescriptor", managed = "Other")]
    pub fn functionDescriptor() -> Id<MTLFunctionDescriptor>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "name", managed = "Other")]
    pub fn name(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setName:")]
    pub fn setName(&self, name: Option<&NSString>);

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "specializedName", managed = "Other")]
    pub fn specializedName(&self) -> Option<Id<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "setSpecializedName:")]
    pub fn setSpecializedName(&self, specialized_name: Option<&NSString>);

    #[cfg(feature = "Metal_MTLFunctionConstantValues")]
    #[objc2::method(sel = "constantValues", managed = "Other")]
    pub fn constantValues(&self) -> Option<Id<MTLFunctionConstantValues>>;

    #[cfg(feature = "Metal_MTLFunctionConstantValues")]
    #[objc2::method(sel = "setConstantValues:")]
    pub fn setConstantValues(&self, constant_values: Option<&MTLFunctionConstantValues>);

    #[objc2::method(sel = "options")]
    pub fn options(&self) -> MTLFunctionOptions;

    #[objc2::method(sel = "setOptions:")]
    pub fn setOptions(&self, options: MTLFunctionOptions);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "binaryArchives", managed = "Other")]
    pub unsafe fn binaryArchives(
        &self,
    ) -> Option<Id<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setBinaryArchives:")]
    pub unsafe fn setBinaryArchives(
        &self,
        binary_archives: Option<&NSArray<ProtocolObject<dyn MTLBinaryArchive>>>,
    );
}

#[objc2::interface(
    unsafe super = MTLFunctionDescriptor,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLIntersectionFunctionDescriptor")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MTLIntersectionFunctionDescriptor;
}

#[cfg(feature = "Metal_MTLIntersectionFunctionDescriptor")]
unsafe impl NSObjectProtocol for MTLIntersectionFunctionDescriptor {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Metal_MTLIntersectionFunctionDescriptor")]
    pub type MTLIntersectionFunctionDescriptor;
}
