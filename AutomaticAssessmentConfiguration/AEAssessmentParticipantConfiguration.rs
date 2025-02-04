//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AutomaticAssessmentConfiguration::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentParticipantConfiguration")]
    pub struct AEAssessmentParticipantConfiguration;

    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentParticipantConfiguration")]
    unsafe impl ClassType for AEAssessmentParticipantConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentParticipantConfiguration")]
unsafe impl NSObjectProtocol for AEAssessmentParticipantConfiguration {}

extern_methods!(
    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentParticipantConfiguration")]
    unsafe impl AEAssessmentParticipantConfiguration {
        #[method(allowsNetworkAccess)]
        pub unsafe fn allowsNetworkAccess(&self) -> bool;

        #[method(setAllowsNetworkAccess:)]
        pub unsafe fn setAllowsNetworkAccess(&self, allows_network_access: bool);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
