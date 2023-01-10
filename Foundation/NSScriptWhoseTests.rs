//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTestComparisonOperation {
        NSEqualToComparison = 0,
        NSLessThanOrEqualToComparison = 1,
        NSLessThanComparison = 2,
        NSGreaterThanOrEqualToComparison = 3,
        NSGreaterThanComparison = 4,
        NSBeginsWithComparison = 5,
        NSEndsWithComparison = 6,
        NSContainsComparison = 7,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptWhoseTest;

    unsafe impl ClassType for NSScriptWhoseTest {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptWhoseTest")]
    unsafe impl NSScriptWhoseTest {
        #[method(isTrue)]
        pub unsafe fn isTrue(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            inCoder: &Foundation::NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLogicalTest;

    unsafe impl ClassType for NSLogicalTest {
        #[inherits(NSObject)]
        type Super = NSScriptWhoseTest;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSLogicalTest")]
    unsafe impl NSLogicalTest {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSSpecifierTest"))]
        #[method_id(@__retain_semantics Init initAndTestWithTests:)]
        pub unsafe fn initAndTestWithTests(
            this: Option<Allocated<Self>>,
            subTests: &Foundation::NSArray<Foundation::NSSpecifierTest>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSSpecifierTest"))]
        #[method_id(@__retain_semantics Init initOrTestWithTests:)]
        pub unsafe fn initOrTestWithTests(
            this: Option<Allocated<Self>>,
            subTests: &Foundation::NSArray<Foundation::NSSpecifierTest>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initNotTestWithTest:)]
        pub unsafe fn initNotTestWithTest(
            this: Option<Allocated<Self>>,
            subTest: &Foundation::NSScriptWhoseTest,
        ) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSpecifierTest;

    unsafe impl ClassType for NSSpecifierTest {
        #[inherits(NSObject)]
        type Super = NSScriptWhoseTest;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSSpecifierTest")]
    unsafe impl NSSpecifierTest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            inCoder: &Foundation::NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method_id(@__retain_semantics Init initWithObjectSpecifier:comparisonOperator:testObject:)]
        pub unsafe fn initWithObjectSpecifier_comparisonOperator_testObject(
            this: Option<Allocated<Self>>,
            obj1: Option<&Foundation::NSScriptObjectSpecifier>,
            compOp: Foundation::NSTestComparisonOperation,
            obj2: Option<&Object>,
        ) -> Id<Self, Shared>;
    }
);
