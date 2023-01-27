//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebDocumentView: NSObjectProtocol {
        #[cfg(feature = "WebKit_WebDataSource")]
        #[method(setDataSource:)]
        unsafe fn setDataSource(&self, data_source: Option<&WebDataSource>);

        #[cfg(feature = "WebKit_WebDataSource")]
        #[method(dataSourceUpdated:)]
        unsafe fn dataSourceUpdated(&self, data_source: Option<&WebDataSource>);

        #[method(setNeedsLayout:)]
        unsafe fn setNeedsLayout(&self, flag: bool);

        #[method(layout)]
        unsafe fn layout(&self);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(viewWillMoveToHostWindow:)]
        unsafe fn viewWillMoveToHostWindow(&self, host_window: Option<&NSWindow>);

        #[method(viewDidMoveToHostWindow)]
        unsafe fn viewDidMoveToHostWindow(&self);
    }

    unsafe impl ProtocolType for dyn WebDocumentView {}
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebDocumentSearching: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method(searchFor:direction:caseSensitive:wrap:)]
        unsafe fn searchFor_direction_caseSensitive_wrap(
            &self,
            string: Option<&NSString>,
            forward: bool,
            case_flag: bool,
            wrap_flag: bool,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn WebDocumentSearching {}
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebDocumentText: NSObjectProtocol {
        #[method(supportsTextEncoding)]
        unsafe fn supportsTextEncoding(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other string)]
        unsafe fn string(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedString)]
        unsafe fn attributedString(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other selectedString)]
        unsafe fn selectedString(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other selectedAttributedString)]
        unsafe fn selectedAttributedString(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[method(selectAll)]
        unsafe fn selectAll(&self);

        #[method(deselectAll)]
        unsafe fn deselectAll(&self);
    }

    unsafe impl ProtocolType for dyn WebDocumentText {}
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebDocumentRepresentation: NSObjectProtocol {
        #[cfg(feature = "WebKit_WebDataSource")]
        #[method(setDataSource:)]
        unsafe fn setDataSource(&self, data_source: Option<&WebDataSource>);

        #[cfg(all(feature = "Foundation_NSData", feature = "WebKit_WebDataSource"))]
        #[method(receivedData:withDataSource:)]
        unsafe fn receivedData_withDataSource(
            &self,
            data: Option<&NSData>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "WebKit_WebDataSource"))]
        #[method(receivedError:withDataSource:)]
        unsafe fn receivedError_withDataSource(
            &self,
            error: Option<&NSError>,
            data_source: Option<&WebDataSource>,
        );

        #[cfg(feature = "WebKit_WebDataSource")]
        #[method(finishedLoadingWithDataSource:)]
        unsafe fn finishedLoadingWithDataSource(&self, data_source: Option<&WebDataSource>);

        #[method(canProvideDocumentSource)]
        unsafe fn canProvideDocumentSource(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other documentSource)]
        unsafe fn documentSource(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        unsafe fn title(&self) -> Option<Id<NSString, Shared>>;
    }

    unsafe impl ProtocolType for dyn WebDocumentRepresentation {}
);
