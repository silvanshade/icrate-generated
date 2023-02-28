//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_WKFindConfiguration")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type WKFindConfiguration;
}

#[cfg(feature = "WebKit_WKFindConfiguration")]
unsafe impl NSObjectProtocol for WKFindConfiguration {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "WebKit_WKFindConfiguration")]
    pub type WKFindConfiguration;

    #[objc2::method(sel = "backwards")]
    pub unsafe fn backwards(&self) -> bool;

    #[objc2::method(sel = "setBackwards:")]
    pub unsafe fn setBackwards(&self, backwards: bool);

    #[objc2::method(sel = "caseSensitive")]
    pub unsafe fn caseSensitive(&self) -> bool;

    #[objc2::method(sel = "setCaseSensitive:")]
    pub unsafe fn setCaseSensitive(&self, case_sensitive: bool);

    #[objc2::method(sel = "wraps")]
    pub unsafe fn wraps(&self) -> bool;

    #[objc2::method(sel = "setWraps:")]
    pub unsafe fn setWraps(&self, wraps: bool);
}
