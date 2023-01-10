//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

pub type NSPageControllerObjectIdentifier = Foundation::NSString;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPageControllerTransitionStyle {
        NSPageControllerTransitionStyleStackHistory = 0,
        NSPageControllerTransitionStyleStackBook = 1,
        NSPageControllerTransitionStyleHorizontalStrip = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPageController;

    unsafe impl ClassType for NSPageController {
        #[inherits(AppKit::NSResponder, NSObject)]
        type Super = AppKit::NSViewController;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSPageController")]
    unsafe impl NSPageController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<AppKit::NSPageControllerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&AppKit::NSPageControllerDelegate>);

        #[method_id(@__retain_semantics Other selectedViewController)]
        pub unsafe fn selectedViewController(&self)
            -> Option<Id<AppKit::NSViewController, Shared>>;

        #[method(transitionStyle)]
        pub unsafe fn transitionStyle(&self) -> AppKit::NSPageControllerTransitionStyle;

        #[method(setTransitionStyle:)]
        pub unsafe fn setTransitionStyle(
            &self,
            transitionStyle: AppKit::NSPageControllerTransitionStyle,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other arrangedObjects)]
        pub unsafe fn arrangedObjects(&self) -> Id<Foundation::NSArray, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setArrangedObjects:)]
        pub unsafe fn setArrangedObjects(&self, arrangedObjects: &Foundation::NSArray);

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selectedIndex: NSInteger);

        #[method(navigateForwardToObject:)]
        pub unsafe fn navigateForwardToObject(&self, object: &Object);

        #[method(completeTransition)]
        pub unsafe fn completeTransition(&self);

        #[method(navigateBack:)]
        pub unsafe fn navigateBack(&self, sender: Option<&Object>);

        #[method(navigateForward:)]
        pub unsafe fn navigateForward(&self, sender: Option<&Object>);

        #[method(takeSelectedIndexFrom:)]
        pub unsafe fn takeSelectedIndexFrom(&self, sender: Option<&Object>);
    }
);

extern_protocol!(
    pub struct NSPageControllerDelegate;

    unsafe impl ProtocolType for NSPageControllerDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other pageController:identifierForObject:)]
        pub unsafe fn pageController_identifierForObject(
            &self,
            pageController: &AppKit::NSPageController,
            object: &Object,
        ) -> Id<AppKit::NSPageControllerObjectIdentifier, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other pageController:viewControllerForIdentifier:)]
        pub unsafe fn pageController_viewControllerForIdentifier(
            &self,
            pageController: &AppKit::NSPageController,
            identifier: &AppKit::NSPageControllerObjectIdentifier,
        ) -> Id<AppKit::NSViewController, Shared>;

        #[optional]
        #[method(pageController:frameForObject:)]
        pub unsafe fn pageController_frameForObject(
            &self,
            pageController: &AppKit::NSPageController,
            object: Option<&Object>,
        ) -> Foundation::NSRect;

        #[optional]
        #[method(pageController:prepareViewController:withObject:)]
        pub unsafe fn pageController_prepareViewController_withObject(
            &self,
            pageController: &AppKit::NSPageController,
            viewController: &AppKit::NSViewController,
            object: Option<&Object>,
        );

        #[optional]
        #[method(pageController:didTransitionToObject:)]
        pub unsafe fn pageController_didTransitionToObject(
            &self,
            pageController: &AppKit::NSPageController,
            object: &Object,
        );

        #[optional]
        #[method(pageControllerWillStartLiveTransition:)]
        pub unsafe fn pageControllerWillStartLiveTransition(
            &self,
            pageController: &AppKit::NSPageController,
        );

        #[optional]
        #[method(pageControllerDidEndLiveTransition:)]
        pub unsafe fn pageControllerDidEndLiveTransition(
            &self,
            pageController: &AppKit::NSPageController,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "AppKit_NSPageController")]
    unsafe impl AppKit::NSPageController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nibNameOrNil: Option<&AppKit::NSNibName>,
            nibBundleOrNil: Option<&Foundation::NSBundle>,
        ) -> Id<Self, Shared>;
    }
);
