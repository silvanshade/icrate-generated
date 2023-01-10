//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStepperTouchBarItem;

    unsafe impl ClassType for NSStepperTouchBarItem {
        #[inherits(NSObject)]
        type Super = AppKit::NSTouchBarItem;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSStepperTouchBarItem")]
    unsafe impl NSStepperTouchBarItem {
        #[cfg(feature = "Foundation_NSFormatter")]
        #[method_id(@__retain_semantics Other stepperTouchBarItemWithIdentifier:formatter:)]
        pub unsafe fn stepperTouchBarItemWithIdentifier_formatter(
            identifier: &AppKit::NSTouchBarItemIdentifier,
            formatter: &Foundation::NSFormatter,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other stepperTouchBarItemWithIdentifier:drawingHandler:)]
        pub unsafe fn stepperTouchBarItemWithIdentifier_drawingHandler(
            identifier: &AppKit::NSTouchBarItemIdentifier,
            drawingHandler: &Block<(Foundation::NSRect, c_double), ()>,
        ) -> Id<Self, Shared>;

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, maxValue: c_double);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, minValue: c_double);

        #[method(increment)]
        pub unsafe fn increment(&self) -> c_double;

        #[method(setIncrement:)]
        pub unsafe fn setIncrement(&self, increment: c_double);

        #[method(value)]
        pub unsafe fn value(&self) -> c_double;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_double);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<Foundation::NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(
            &self,
            customizationLabel: Option<&Foundation::NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSStepperTouchBarItem")]
    unsafe impl AppKit::NSStepperTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &AppKit::NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
    }
);
