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
    #[cfg(feature = "Foundation_NSURLCredentialStorage")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSURLCredentialStorage;
}

#[cfg(feature = "Foundation_NSURLCredentialStorage")]
unsafe impl NSObjectProtocol for NSURLCredentialStorage {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSURLCredentialStorage")]
    pub type NSURLCredentialStorage;

    #[objc2::method(sel = "sharedCredentialStorage", managed = "Other")]
    pub unsafe fn sharedCredentialStorage() -> Id<NSURLCredentialStorage>;

    #[cfg(all(
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace"
    ))]
    #[objc2::method(sel = "credentialsForProtectionSpace:", managed = "Other")]
    pub unsafe fn credentialsForProtectionSpace(
        &self,
        space: &NSURLProtectionSpace,
    ) -> Option<Id<NSDictionary<NSString, NSURLCredential>>>;

    #[cfg(all(
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace"
    ))]
    #[objc2::method(sel = "allCredentials", managed = "Other")]
    pub unsafe fn allCredentials(
        &self,
    ) -> Id<NSDictionary<NSURLProtectionSpace, NSDictionary<NSString, NSURLCredential>>>;

    #[cfg(all(
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace"
    ))]
    #[objc2::method(sel = "setCredential:forProtectionSpace:")]
    pub unsafe fn setCredential_forProtectionSpace(
        &self,
        credential: &NSURLCredential,
        space: &NSURLProtectionSpace,
    );

    #[cfg(all(
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace"
    ))]
    #[objc2::method(sel = "removeCredential:forProtectionSpace:")]
    pub unsafe fn removeCredential_forProtectionSpace(
        &self,
        credential: &NSURLCredential,
        space: &NSURLProtectionSpace,
    );

    #[cfg(all(
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace"
    ))]
    #[objc2::method(sel = "removeCredential:forProtectionSpace:options:")]
    pub unsafe fn removeCredential_forProtectionSpace_options(
        &self,
        credential: &NSURLCredential,
        space: &NSURLProtectionSpace,
        options: Option<&NSDictionary<NSString, Object>>,
    );

    #[cfg(all(
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace"
    ))]
    #[objc2::method(sel = "defaultCredentialForProtectionSpace:", managed = "Other")]
    pub unsafe fn defaultCredentialForProtectionSpace(
        &self,
        space: &NSURLProtectionSpace,
    ) -> Option<Id<NSURLCredential>>;

    #[cfg(all(
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace"
    ))]
    #[objc2::method(sel = "setDefaultCredential:forProtectionSpace:")]
    pub unsafe fn setDefaultCredential_forProtectionSpace(
        &self,
        credential: &NSURLCredential,
        space: &NSURLProtectionSpace,
    );
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Foundation_NSURLCredentialStorage")]
    pub type NSURLCredentialStorage;

    #[cfg(all(
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace",
        feature = "Foundation_NSURLSessionTask"
    ))]
    #[objc2::method(sel = "getCredentialsForProtectionSpace:task:completionHandler:")]
    pub unsafe fn getCredentialsForProtectionSpace_task_completionHandler(
        &self,
        protection_space: &NSURLProtectionSpace,
        task: &NSURLSessionTask,
        completion_handler: &Block<(*mut NSDictionary<NSString, NSURLCredential>,), ()>,
    );

    #[cfg(all(
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace",
        feature = "Foundation_NSURLSessionTask"
    ))]
    #[objc2::method(sel = "setCredential:forProtectionSpace:task:")]
    pub unsafe fn setCredential_forProtectionSpace_task(
        &self,
        credential: &NSURLCredential,
        protection_space: &NSURLProtectionSpace,
        task: &NSURLSessionTask,
    );

    #[cfg(all(
        feature = "Foundation_NSDictionary",
        feature = "Foundation_NSString",
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace",
        feature = "Foundation_NSURLSessionTask"
    ))]
    #[objc2::method(sel = "removeCredential:forProtectionSpace:options:task:")]
    pub unsafe fn removeCredential_forProtectionSpace_options_task(
        &self,
        credential: &NSURLCredential,
        protection_space: &NSURLProtectionSpace,
        options: Option<&NSDictionary<NSString, Object>>,
        task: &NSURLSessionTask,
    );

    #[cfg(all(
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace",
        feature = "Foundation_NSURLSessionTask"
    ))]
    #[objc2::method(sel = "getDefaultCredentialForProtectionSpace:task:completionHandler:")]
    pub unsafe fn getDefaultCredentialForProtectionSpace_task_completionHandler(
        &self,
        space: &NSURLProtectionSpace,
        task: &NSURLSessionTask,
        completion_handler: &Block<(*mut NSURLCredential,), ()>,
    );

    #[cfg(all(
        feature = "Foundation_NSURLCredential",
        feature = "Foundation_NSURLProtectionSpace",
        feature = "Foundation_NSURLSessionTask"
    ))]
    #[objc2::method(sel = "setDefaultCredential:forProtectionSpace:task:")]
    pub unsafe fn setDefaultCredential_forProtectionSpace_task(
        &self,
        credential: &NSURLCredential,
        protection_space: &NSURLProtectionSpace,
        task: &NSURLSessionTask,
    );
}

extern_static!(NSURLCredentialStorageChangedNotification: &'static NSNotificationName);

extern_static!(NSURLCredentialStorageRemoveSynchronizableCredentials: &'static NSString);
