//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSNotificationName = NSString;
);

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSNotification")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSNotification;
}

#[cfg(feature = "Foundation_NSNotification")]
unsafe impl NSCoding for NSNotification {}

#[cfg(feature = "Foundation_NSNotification")]
unsafe impl NSObjectProtocol for NSNotification {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSNotification")]
    pub type NSNotification;

    #[objc2::method(sel = "name", managed = "Other")]
    pub unsafe fn name(&self) -> Id<NSNotificationName>;

    #[objc2::method(sel = "object", managed = "Other")]
    pub unsafe fn object(&self) -> Option<Id<Object>>;

    #[cfg(feature = "Foundation_NSDictionary")]
    #[objc2::method(sel = "userInfo", managed = "Other")]
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary>>;

    #[cfg(feature = "Foundation_NSDictionary")]
    #[objc2::method(sel = "initWithName:object:userInfo:", managed = "Init")]
    pub unsafe fn initWithName_object_userInfo(
        this: Option<Allocated<Self>>,
        name: &NSNotificationName,
        object: Option<&Object>,
        user_info: Option<&NSDictionary>,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSCoder")]
    #[objc2::method(sel = "initWithCoder:", managed = "Init")]
    pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder)
        -> Option<Id<Self>>;
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSNotification")]
    pub type NSNotification;

    #[objc2::method(sel = "notificationWithName:object:", managed = "Other")]
    pub unsafe fn notificationWithName_object(
        a_name: &NSNotificationName,
        an_object: Option<&Object>,
    ) -> Id<Self>;

    #[cfg(feature = "Foundation_NSDictionary")]
    #[objc2::method(sel = "notificationWithName:object:userInfo:", managed = "Other")]
    pub unsafe fn notificationWithName_object_userInfo(
        a_name: &NSNotificationName,
        an_object: Option<&Object>,
        a_user_info: Option<&NSDictionary>,
    ) -> Id<Self>;

    #[objc2::method(sel = "init", managed = "Init")]
    pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSNotificationCenter")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSNotificationCenter;
}

#[cfg(feature = "Foundation_NSNotificationCenter")]
unsafe impl NSObjectProtocol for NSNotificationCenter {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSNotificationCenter")]
    pub type NSNotificationCenter;

    #[objc2::method(sel = "defaultCenter", managed = "Other")]
    pub unsafe fn defaultCenter() -> Id<NSNotificationCenter>;

    #[objc2::method(sel = "addObserver:selector:name:object:")]
    pub unsafe fn addObserver_selector_name_object(
        &self,
        observer: &Object,
        a_selector: Sel,
        a_name: Option<&NSNotificationName>,
        an_object: Option<&Object>,
    );

    #[cfg(feature = "Foundation_NSNotification")]
    #[objc2::method(sel = "postNotification:")]
    pub unsafe fn postNotification(&self, notification: &NSNotification);

    #[objc2::method(sel = "postNotificationName:object:")]
    pub unsafe fn postNotificationName_object(
        &self,
        a_name: &NSNotificationName,
        an_object: Option<&Object>,
    );

    #[cfg(feature = "Foundation_NSDictionary")]
    #[objc2::method(sel = "postNotificationName:object:userInfo:")]
    pub unsafe fn postNotificationName_object_userInfo(
        &self,
        a_name: &NSNotificationName,
        an_object: Option<&Object>,
        a_user_info: Option<&NSDictionary>,
    );

    #[objc2::method(sel = "removeObserver:")]
    pub unsafe fn removeObserver(&self, observer: &Object);

    #[objc2::method(sel = "removeObserver:name:object:")]
    pub unsafe fn removeObserver_name_object(
        &self,
        observer: &Object,
        a_name: Option<&NSNotificationName>,
        an_object: Option<&Object>,
    );

    #[cfg(all(
        feature = "Foundation_NSNotification",
        feature = "Foundation_NSOperationQueue"
    ))]
    #[objc2::method(sel = "addObserverForName:object:queue:usingBlock:", managed = "Other")]
    pub unsafe fn addObserverForName_object_queue_usingBlock(
        &self,
        name: Option<&NSNotificationName>,
        obj: Option<&Object>,
        queue: Option<&NSOperationQueue>,
        block: &Block<(NonNull<NSNotification>,), ()>,
    ) -> Id<NSObject>;
}
