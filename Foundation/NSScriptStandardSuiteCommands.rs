//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSSaveOptions {
        NSSaveOptionsYes = 0,
        NSSaveOptionsNo = 1,
        NSSaveOptionsAsk = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCloneCommand;

    unsafe impl ClassType for NSCloneCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSCloneCommand")]
    unsafe impl NSCloneCommand {
        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(
            &self,
            receiversRef: Option<&Foundation::NSScriptObjectSpecifier>,
        );

        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<Foundation::NSScriptObjectSpecifier, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCloseCommand;

    unsafe impl ClassType for NSCloseCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSCloseCommand")]
    unsafe impl NSCloseCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> Foundation::NSSaveOptions;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCountCommand;

    unsafe impl ClassType for NSCountCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSCountCommand")]
    unsafe impl NSCountCommand {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCreateCommand;

    unsafe impl ClassType for NSCreateCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSCreateCommand")]
    unsafe impl NSCreateCommand {
        #[cfg(feature = "Foundation_NSScriptClassDescription")]
        #[method_id(@__retain_semantics Other createClassDescription)]
        pub unsafe fn createClassDescription(
            &self,
        ) -> Id<Foundation::NSScriptClassDescription, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other resolvedKeyDictionary)]
        pub unsafe fn resolvedKeyDictionary(
            &self,
        ) -> Id<Foundation::NSDictionary<Foundation::NSString, Object>, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDeleteCommand;

    unsafe impl ClassType for NSDeleteCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSDeleteCommand")]
    unsafe impl NSDeleteCommand {
        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(
            &self,
            receiversRef: Option<&Foundation::NSScriptObjectSpecifier>,
        );

        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<Foundation::NSScriptObjectSpecifier, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSExistsCommand;

    unsafe impl ClassType for NSExistsCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSExistsCommand")]
    unsafe impl NSExistsCommand {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGetCommand;

    unsafe impl ClassType for NSGetCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSGetCommand")]
    unsafe impl NSGetCommand {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMoveCommand;

    unsafe impl ClassType for NSMoveCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMoveCommand")]
    unsafe impl NSMoveCommand {
        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(
            &self,
            receiversRef: Option<&Foundation::NSScriptObjectSpecifier>,
        );

        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<Foundation::NSScriptObjectSpecifier, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSQuitCommand;

    unsafe impl ClassType for NSQuitCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSQuitCommand")]
    unsafe impl NSQuitCommand {
        #[method(saveOptions)]
        pub unsafe fn saveOptions(&self) -> Foundation::NSSaveOptions;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSetCommand;

    unsafe impl ClassType for NSSetCommand {
        #[inherits(NSObject)]
        type Super = Foundation::NSScriptCommand;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSSetCommand")]
    unsafe impl NSSetCommand {
        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method(setReceiversSpecifier:)]
        pub unsafe fn setReceiversSpecifier(
            &self,
            receiversRef: Option<&Foundation::NSScriptObjectSpecifier>,
        );

        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method_id(@__retain_semantics Other keySpecifier)]
        pub unsafe fn keySpecifier(&self) -> Id<Foundation::NSScriptObjectSpecifier, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSCloneCommand")]
    unsafe impl Foundation::NSCloneCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSCloseCommand")]
    unsafe impl Foundation::NSCloseCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSCountCommand")]
    unsafe impl Foundation::NSCountCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSCreateCommand")]
    unsafe impl Foundation::NSCreateCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSDeleteCommand")]
    unsafe impl Foundation::NSDeleteCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSExistsCommand")]
    unsafe impl Foundation::NSExistsCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSGetCommand")]
    unsafe impl Foundation::NSGetCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSMoveCommand")]
    unsafe impl Foundation::NSMoveCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSQuitCommand")]
    unsafe impl Foundation::NSQuitCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSScriptCommand`
    #[cfg(feature = "Foundation_NSSetCommand")]
    unsafe impl Foundation::NSSetCommand {
        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Init initWithCommandDescription:)]
        pub unsafe fn initWithCommandDescription(
            this: Option<Allocated<Self>>,
            commandDef: &Foundation::NSScriptCommandDescription,
        ) -> Id<Self, Shared>;
    }
);
