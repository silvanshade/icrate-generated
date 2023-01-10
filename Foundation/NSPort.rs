//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

pub type NSSocketNativeHandle = c_int;

extern_static!(NSPortDidBecomeInvalidNotification: &'static Foundation::NSNotificationName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPort;

    unsafe impl ClassType for NSPort {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSPort")]
    unsafe impl NSPort {
        #[method_id(@__retain_semantics Other port)]
        pub unsafe fn port() -> Id<Foundation::NSPort, Shared>;

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, anObject: Option<&Foundation::NSPortDelegate>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<Foundation::NSPortDelegate, Shared>>;

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            runLoop: &Foundation::NSRunLoop,
            mode: &Foundation::NSRunLoopMode,
        );

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(
            &self,
            runLoop: &Foundation::NSRunLoop,
            mode: &Foundation::NSRunLoopMode,
        );

        #[method(reservedSpaceLength)]
        pub unsafe fn reservedSpaceLength(&self) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSMutableArray"))]
        #[method(sendBeforeDate:components:from:reserved:)]
        pub unsafe fn sendBeforeDate_components_from_reserved(
            &self,
            limitDate: &Foundation::NSDate,
            components: Option<&Foundation::NSMutableArray>,
            receivePort: Option<&Foundation::NSPort>,
            headerSpaceReserved: NSUInteger,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSMutableArray"))]
        #[method(sendBeforeDate:msgid:components:from:reserved:)]
        pub unsafe fn sendBeforeDate_msgid_components_from_reserved(
            &self,
            limitDate: &Foundation::NSDate,
            msgID: NSUInteger,
            components: Option<&Foundation::NSMutableArray>,
            receivePort: Option<&Foundation::NSPort>,
            headerSpaceReserved: NSUInteger,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSConnection", feature = "Foundation_NSRunLoop"))]
        #[method(addConnection:toRunLoop:forMode:)]
        pub unsafe fn addConnection_toRunLoop_forMode(
            &self,
            conn: &Foundation::NSConnection,
            runLoop: &Foundation::NSRunLoop,
            mode: &Foundation::NSRunLoopMode,
        );

        #[cfg(all(feature = "Foundation_NSConnection", feature = "Foundation_NSRunLoop"))]
        #[method(removeConnection:fromRunLoop:forMode:)]
        pub unsafe fn removeConnection_fromRunLoop_forMode(
            &self,
            conn: &Foundation::NSConnection,
            runLoop: &Foundation::NSRunLoop,
            mode: &Foundation::NSRunLoopMode,
        );
    }
);

extern_protocol!(
    pub struct NSPortDelegate;

    unsafe impl ProtocolType for NSPortDelegate {
        #[optional]
        #[method(handlePortMessage:)]
        pub unsafe fn handlePortMessage(&self, message: &Foundation::NSPortMessage);
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMachPortOptions {
        NSMachPortDeallocateNone = 0,
        NSMachPortDeallocateSendRight = 1 << 0,
        NSMachPortDeallocateReceiveRight = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMachPort;

    unsafe impl ClassType for NSMachPort {
        #[inherits(NSObject)]
        type Super = NSPort;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMachPort")]
    unsafe impl NSMachPort {
        #[method_id(@__retain_semantics Other portWithMachPort:)]
        pub unsafe fn portWithMachPort(machPort: u32) -> Id<Foundation::NSPort, Shared>;

        #[method_id(@__retain_semantics Init initWithMachPort:)]
        pub unsafe fn initWithMachPort(
            this: Option<Allocated<Self>>,
            machPort: u32,
        ) -> Id<Self, Shared>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, anObject: Option<&Foundation::NSMachPortDelegate>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<Foundation::NSMachPortDelegate, Shared>>;

        #[method_id(@__retain_semantics Other portWithMachPort:options:)]
        pub unsafe fn portWithMachPort_options(
            machPort: u32,
            f: Foundation::NSMachPortOptions,
        ) -> Id<Foundation::NSPort, Shared>;

        #[method_id(@__retain_semantics Init initWithMachPort:options:)]
        pub unsafe fn initWithMachPort_options(
            this: Option<Allocated<Self>>,
            machPort: u32,
            f: Foundation::NSMachPortOptions,
        ) -> Id<Self, Shared>;

        #[method(machPort)]
        pub unsafe fn machPort(&self) -> u32;

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            runLoop: &Foundation::NSRunLoop,
            mode: &Foundation::NSRunLoopMode,
        );

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(
            &self,
            runLoop: &Foundation::NSRunLoop,
            mode: &Foundation::NSRunLoopMode,
        );
    }
);

extern_protocol!(
    pub struct NSMachPortDelegate;

    unsafe impl ProtocolType for NSMachPortDelegate {
        #[optional]
        #[method(handleMachMessage:)]
        pub unsafe fn handleMachMessage(&self, msg: NonNull<c_void>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMessagePort;

    unsafe impl ClassType for NSMessagePort {
        #[inherits(NSObject)]
        type Super = NSPort;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMessagePort")]
    unsafe impl NSMessagePort {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSocketPort;

    unsafe impl ClassType for NSSocketPort {
        #[inherits(NSObject)]
        type Super = NSPort;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSSocketPort")]
    unsafe impl NSSocketPort {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithTCPPort:)]
        pub unsafe fn initWithTCPPort(
            this: Option<Allocated<Self>>,
            port: c_ushort,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithProtocolFamily:socketType:protocol:address:)]
        pub unsafe fn initWithProtocolFamily_socketType_protocol_address(
            this: Option<Allocated<Self>>,
            family: c_int,
            type_: c_int,
            protocol: c_int,
            address: &Foundation::NSData,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithProtocolFamily:socketType:protocol:socket:)]
        pub unsafe fn initWithProtocolFamily_socketType_protocol_socket(
            this: Option<Allocated<Self>>,
            family: c_int,
            type_: c_int,
            protocol: c_int,
            sock: Foundation::NSSocketNativeHandle,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initRemoteWithTCPPort:host:)]
        pub unsafe fn initRemoteWithTCPPort_host(
            this: Option<Allocated<Self>>,
            port: c_ushort,
            hostName: Option<&Foundation::NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initRemoteWithProtocolFamily:socketType:protocol:address:)]
        pub unsafe fn initRemoteWithProtocolFamily_socketType_protocol_address(
            this: Option<Allocated<Self>>,
            family: c_int,
            type_: c_int,
            protocol: c_int,
            address: &Foundation::NSData,
        ) -> Id<Self, Shared>;

        #[method(protocolFamily)]
        pub unsafe fn protocolFamily(&self) -> c_int;

        #[method(socketType)]
        pub unsafe fn socketType(&self) -> c_int;

        #[method(protocol)]
        pub unsafe fn protocol(&self) -> c_int;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other address)]
        pub unsafe fn address(&self) -> Id<Foundation::NSData, Shared>;

        #[method(socket)]
        pub unsafe fn socket(&self) -> Foundation::NSSocketNativeHandle;
    }
);
