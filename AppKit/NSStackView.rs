//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSStackViewGravity {
        NSStackViewGravityTop = 1,
        NSStackViewGravityLeading = 1,
        NSStackViewGravityCenter = 2,
        NSStackViewGravityBottom = 3,
        NSStackViewGravityTrailing = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSStackViewDistribution {
        NSStackViewDistributionGravityAreas = -1,
        NSStackViewDistributionFill = 0,
        NSStackViewDistributionFillEqually = 1,
        NSStackViewDistributionFillProportionally = 2,
        NSStackViewDistributionEqualSpacing = 3,
        NSStackViewDistributionEqualCentering = 4,
    }
);

typed_extensible_enum!(
    pub type NSStackViewVisibilityPriority = c_float;
);

extern_static!(NSStackViewVisibilityPriorityMustHold: NSStackViewVisibilityPriority = 1000);

extern_static!(
    NSStackViewVisibilityPriorityDetachOnlyIfNecessary: NSStackViewVisibilityPriority = 900
);

extern_static!(NSStackViewVisibilityPriorityNotVisible: NSStackViewVisibilityPriority = 0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSStackView")]
    pub struct NSStackView;

    #[cfg(feature = "AppKit_NSStackView")]
    unsafe impl ClassType for NSStackView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSStackView")]
    unsafe impl NSStackView {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other stackViewWithViews:)]
        pub unsafe fn stackViewWithViews(views: &NSArray<NSView>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSStackViewDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSStackViewDelegate>);

        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSUserInterfaceLayoutOrientation;

        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSUserInterfaceLayoutOrientation);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSLayoutAttribute;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSLayoutAttribute);

        #[method(edgeInsets)]
        pub unsafe fn edgeInsets(&self) -> NSEdgeInsets;

        #[method(setEdgeInsets:)]
        pub unsafe fn setEdgeInsets(&self, edgeInsets: NSEdgeInsets);

        #[method(distribution)]
        pub unsafe fn distribution(&self) -> NSStackViewDistribution;

        #[method(setDistribution:)]
        pub unsafe fn setDistribution(&self, distribution: NSStackViewDistribution);

        #[method(spacing)]
        pub unsafe fn spacing(&self) -> CGFloat;

        #[method(setSpacing:)]
        pub unsafe fn setSpacing(&self, spacing: CGFloat);

        #[method(setCustomSpacing:afterView:)]
        pub unsafe fn setCustomSpacing_afterView(&self, spacing: CGFloat, view: &NSView);

        #[method(customSpacingAfterView:)]
        pub unsafe fn customSpacingAfterView(&self, view: &NSView) -> CGFloat;

        #[method(detachesHiddenViews)]
        pub unsafe fn detachesHiddenViews(&self) -> bool;

        #[method(setDetachesHiddenViews:)]
        pub unsafe fn setDetachesHiddenViews(&self, detachesHiddenViews: bool);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other arrangedSubviews)]
        pub unsafe fn arrangedSubviews(&self) -> Id<NSArray<NSView>, Shared>;

        #[method(addArrangedSubview:)]
        pub unsafe fn addArrangedSubview(&self, view: &NSView);

        #[method(insertArrangedSubview:atIndex:)]
        pub unsafe fn insertArrangedSubview_atIndex(&self, view: &NSView, index: NSInteger);

        #[method(removeArrangedSubview:)]
        pub unsafe fn removeArrangedSubview(&self, view: &NSView);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other detachedViews)]
        pub unsafe fn detachedViews(&self) -> Id<NSArray<NSView>, Shared>;

        #[method(setVisibilityPriority:forView:)]
        pub unsafe fn setVisibilityPriority_forView(
            &self,
            priority: NSStackViewVisibilityPriority,
            view: &NSView,
        );

        #[method(visibilityPriorityForView:)]
        pub unsafe fn visibilityPriorityForView(
            &self,
            view: &NSView,
        ) -> NSStackViewVisibilityPriority;

        #[method(clippingResistancePriorityForOrientation:)]
        pub unsafe fn clippingResistancePriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[method(setClippingResistancePriority:forOrientation:)]
        pub unsafe fn setClippingResistancePriority_forOrientation(
            &self,
            clippingResistancePriority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[method(huggingPriorityForOrientation:)]
        pub unsafe fn huggingPriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[method(setHuggingPriority:forOrientation:)]
        pub unsafe fn setHuggingPriority_forOrientation(
            &self,
            huggingPriority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );
    }
);

extern_protocol!(
    pub struct NSStackViewDelegate;

    unsafe impl ProtocolType for NSStackViewDelegate {
        #[cfg(all(
            feature = "AppKit_NSStackView",
            feature = "AppKit_NSView",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method(stackView:willDetachViews:)]
        pub unsafe fn stackView_willDetachViews(
            &self,
            stackView: &NSStackView,
            views: &NSArray<NSView>,
        );

        #[cfg(all(
            feature = "AppKit_NSStackView",
            feature = "AppKit_NSView",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method(stackView:didReattachViews:)]
        pub unsafe fn stackView_didReattachViews(
            &self,
            stackView: &NSStackView,
            views: &NSArray<NSView>,
        );
    }
);

extern_methods!(
    /// NSStackViewGravityAreas
    #[cfg(feature = "AppKit_NSStackView")]
    unsafe impl NSStackView {
        #[cfg(feature = "AppKit_NSView")]
        #[method(addView:inGravity:)]
        pub unsafe fn addView_inGravity(&self, view: &NSView, gravity: NSStackViewGravity);

        #[cfg(feature = "AppKit_NSView")]
        #[method(insertView:atIndex:inGravity:)]
        pub unsafe fn insertView_atIndex_inGravity(
            &self,
            view: &NSView,
            index: NSUInteger,
            gravity: NSStackViewGravity,
        );

        #[cfg(feature = "AppKit_NSView")]
        #[method(removeView:)]
        pub unsafe fn removeView(&self, view: &NSView);

        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other viewsInGravity:)]
        pub unsafe fn viewsInGravity(
            &self,
            gravity: NSStackViewGravity,
        ) -> Id<NSArray<NSView>, Shared>;

        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSArray"))]
        #[method(setViews:inGravity:)]
        pub unsafe fn setViews_inGravity(
            &self,
            views: &NSArray<NSView>,
            gravity: NSStackViewGravity,
        );

        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other views)]
        pub unsafe fn views(&self) -> Id<NSArray<NSView>, Shared>;
    }
);

extern_methods!(
    /// NSStackViewDeprecated
    #[cfg(feature = "AppKit_NSStackView")]
    unsafe impl NSStackView {
        #[deprecated = "Set -distribution to NSStackViewDistributionEqualSpacing instead."]
        #[method(hasEqualSpacing)]
        pub unsafe fn hasEqualSpacing(&self) -> bool;

        #[deprecated = "Set -distribution to NSStackViewDistributionEqualSpacing instead."]
        #[method(setHasEqualSpacing:)]
        pub unsafe fn setHasEqualSpacing(&self, hasEqualSpacing: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSStackView")]
    unsafe impl NSStackView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
