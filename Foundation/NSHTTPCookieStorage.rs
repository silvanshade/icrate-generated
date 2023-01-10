//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSHTTPCookieAcceptPolicy {
        NSHTTPCookieAcceptPolicyAlways = 0,
        NSHTTPCookieAcceptPolicyNever = 1,
        NSHTTPCookieAcceptPolicyOnlyFromMainDocumentDomain = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHTTPCookieStorage;

    unsafe impl ClassType for NSHTTPCookieStorage {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSHTTPCookieStorage")]
    unsafe impl NSHTTPCookieStorage {
        #[method_id(@__retain_semantics Other sharedHTTPCookieStorage)]
        pub unsafe fn sharedHTTPCookieStorage() -> Id<Foundation::NSHTTPCookieStorage, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sharedCookieStorageForGroupContainerIdentifier:)]
        pub unsafe fn sharedCookieStorageForGroupContainerIdentifier(
            identifier: &Foundation::NSString,
        ) -> Id<Foundation::NSHTTPCookieStorage, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSHTTPCookie"))]
        #[method_id(@__retain_semantics Other cookies)]
        pub unsafe fn cookies(
            &self,
        ) -> Option<Id<Foundation::NSArray<Foundation::NSHTTPCookie>, Shared>>;

        #[cfg(feature = "Foundation_NSHTTPCookie")]
        #[method(setCookie:)]
        pub unsafe fn setCookie(&self, cookie: &Foundation::NSHTTPCookie);

        #[cfg(feature = "Foundation_NSHTTPCookie")]
        #[method(deleteCookie:)]
        pub unsafe fn deleteCookie(&self, cookie: &Foundation::NSHTTPCookie);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(removeCookiesSinceDate:)]
        pub unsafe fn removeCookiesSinceDate(&self, date: &Foundation::NSDate);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other cookiesForURL:)]
        pub unsafe fn cookiesForURL(
            &self,
            URL: &Foundation::NSURL,
        ) -> Option<Id<Foundation::NSArray<Foundation::NSHTTPCookie>, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURL"
        ))]
        #[method(setCookies:forURL:mainDocumentURL:)]
        pub unsafe fn setCookies_forURL_mainDocumentURL(
            &self,
            cookies: &Foundation::NSArray<Foundation::NSHTTPCookie>,
            URL: Option<&Foundation::NSURL>,
            mainDocumentURL: Option<&Foundation::NSURL>,
        );

        #[method(cookieAcceptPolicy)]
        pub unsafe fn cookieAcceptPolicy(&self) -> Foundation::NSHTTPCookieAcceptPolicy;

        #[method(setCookieAcceptPolicy:)]
        pub unsafe fn setCookieAcceptPolicy(
            &self,
            cookieAcceptPolicy: Foundation::NSHTTPCookieAcceptPolicy,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortedCookiesUsingDescriptors:)]
        pub unsafe fn sortedCookiesUsingDescriptors(
            &self,
            sortOrder: &Foundation::NSArray<Foundation::NSSortDescriptor>,
        ) -> Id<Foundation::NSArray<Foundation::NSHTTPCookie>, Shared>;
    }
);

extern_methods!(
    /// NSURLSessionTaskAdditions
    #[cfg(feature = "Foundation_NSHTTPCookieStorage")]
    unsafe impl NSHTTPCookieStorage {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(storeCookies:forTask:)]
        pub unsafe fn storeCookies_forTask(
            &self,
            cookies: &Foundation::NSArray<Foundation::NSHTTPCookie>,
            task: &Foundation::NSURLSessionTask,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSHTTPCookie",
            feature = "Foundation_NSURLSessionTask"
        ))]
        #[method(getCookiesForTask:completionHandler:)]
        pub unsafe fn getCookiesForTask_completionHandler(
            &self,
            task: &Foundation::NSURLSessionTask,
            completionHandler: &Block<(*mut Foundation::NSArray<Foundation::NSHTTPCookie>,), ()>,
        );
    }
);

extern_static!(
    NSHTTPCookieManagerAcceptPolicyChangedNotification: &'static Foundation::NSNotificationName
);

extern_static!(
    NSHTTPCookieManagerCookiesChangedNotification: &'static Foundation::NSNotificationName
);
