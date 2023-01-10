//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

typed_extensible_enum!(
    pub type NSTouchBarItemIdentifier = Foundation::NSString;
);

typed_extensible_enum!(
    pub type NSTouchBarItemPriority = c_float;
);

extern_static!(NSTouchBarItemPriorityHigh: AppKit::NSTouchBarItemPriority = 1000);

extern_static!(NSTouchBarItemPriorityNormal: AppKit::NSTouchBarItemPriority = 0);

extern_static!(NSTouchBarItemPriorityLow: AppKit::NSTouchBarItemPriority = -1000);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTouchBarItem;

    unsafe impl ClassType for NSTouchBarItem {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    unsafe impl NSTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &AppKit::NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &Foundation::NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<AppKit::NSTouchBarItemIdentifier, Shared>;

        #[method(visibilityPriority)]
        pub unsafe fn visibilityPriority(&self) -> AppKit::NSTouchBarItemPriority;

        #[method(setVisibilityPriority:)]
        pub unsafe fn setVisibilityPriority(
            &self,
            visibilityPriority: AppKit::NSTouchBarItemPriority,
        );

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<AppKit::NSView, Shared>>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self) -> Option<Id<AppKit::NSViewController, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<Foundation::NSString, Shared>;

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
    }
);

extern_static!(NSTouchBarItemIdentifierFixedSpaceSmall: &'static AppKit::NSTouchBarItemIdentifier);

extern_static!(NSTouchBarItemIdentifierFixedSpaceLarge: &'static AppKit::NSTouchBarItemIdentifier);

extern_static!(NSTouchBarItemIdentifierFlexibleSpace: &'static AppKit::NSTouchBarItemIdentifier);

extern_static!(NSTouchBarItemIdentifierOtherItemsProxy: &'static AppKit::NSTouchBarItemIdentifier);
