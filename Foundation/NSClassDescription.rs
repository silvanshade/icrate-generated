//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSClassDescription")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSClassDescription;
}

#[cfg(feature = "Foundation_NSClassDescription")]
unsafe impl NSObjectProtocol for NSClassDescription {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSClassDescription")]
    pub type NSClassDescription;

    #[objc2::method(sel = "registerClassDescription:forClass:")]
    pub unsafe fn registerClassDescription_forClass(
        description: &NSClassDescription,
        a_class: &Class,
    );

    #[objc2::method(sel = "invalidateClassDescriptionCache")]
    pub unsafe fn invalidateClassDescriptionCache();

    #[objc2::method(sel = "classDescriptionForClass:", managed = "Other")]
    pub unsafe fn classDescriptionForClass(a_class: &Class) -> Option<Id<NSClassDescription>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "attributeKeys", managed = "Other")]
    pub unsafe fn attributeKeys(&self) -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "toOneRelationshipKeys", managed = "Other")]
    pub unsafe fn toOneRelationshipKeys(&self) -> Id<NSArray<NSString>>;

    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    #[objc2::method(sel = "toManyRelationshipKeys", managed = "Other")]
    pub unsafe fn toManyRelationshipKeys(&self) -> Id<NSArray<NSString>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "inverseForRelationshipKey:", managed = "Other")]
    pub unsafe fn inverseForRelationshipKey(
        &self,
        relationship_key: &NSString,
    ) -> Option<Id<NSString>>;
}

extern_static!(NSClassDescriptionNeededForClassNotification: &'static NSNotificationName);
