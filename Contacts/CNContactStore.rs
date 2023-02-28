//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum CNEntityType {
    CNEntityTypeContacts = 0,
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum CNAuthorizationStatus {
    CNAuthorizationStatusNotDetermined = 0,
    CNAuthorizationStatusRestricted = 1,
    CNAuthorizationStatusDenied = 2,
    CNAuthorizationStatusAuthorized = 3,
}

#[objc2::interface(
    unsafe super = NSObject,
    unsafe inherits = [
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "Contacts_CNContactStore")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type CNContactStore;
}

#[cfg(feature = "Contacts_CNContactStore")]
unsafe impl NSObjectProtocol for CNContactStore {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "Contacts_CNContactStore")]
    pub type CNContactStore;

    #[objc2::method(sel = "authorizationStatusForEntityType:")]
    pub unsafe fn authorizationStatusForEntityType(
        entity_type: CNEntityType,
    ) -> CNAuthorizationStatus;

    #[cfg(feature = "Foundation_NSError")]
    #[objc2::method(sel = "requestAccessForEntityType:completionHandler:")]
    pub unsafe fn requestAccessForEntityType_completionHandler(
        &self,
        entity_type: CNEntityType,
        completion_handler: &Block<(Bool, *mut NSError), ()>,
    );

    #[cfg(all(
        feature = "Contacts_CNContact",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError",
        feature = "Foundation_NSPredicate"
    ))]
    #[objc2::method(
        sel = "unifiedContactsMatchingPredicate:keysToFetch:error:",
        managed = "Other",
        throws
    )]
    pub unsafe fn unifiedContactsMatchingPredicate_keysToFetch_error(
        &self,
        predicate: &NSPredicate,
        keys: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
    ) -> Result<Id<NSArray<CNContact>>, Id<NSError>>;

    #[cfg(all(
        feature = "Contacts_CNContact",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError",
        feature = "Foundation_NSString"
    ))]
    #[objc2::method(
        sel = "unifiedContactWithIdentifier:keysToFetch:error:",
        managed = "Other",
        throws
    )]
    pub unsafe fn unifiedContactWithIdentifier_keysToFetch_error(
        &self,
        identifier: &NSString,
        keys: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
    ) -> Result<Id<CNContact>, Id<NSError>>;

    #[cfg(all(
        feature = "Contacts_CNContact",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError"
    ))]
    #[objc2::method(
        sel = "unifiedMeContactWithKeysToFetch:error:",
        managed = "Other",
        throws
    )]
    pub unsafe fn unifiedMeContactWithKeysToFetch_error(
        &self,
        keys: &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
    ) -> Result<Id<CNContact>, Id<NSError>>;

    #[cfg(all(
        feature = "Contacts_CNContact",
        feature = "Contacts_CNContactFetchRequest",
        feature = "Contacts_CNFetchResult",
        feature = "Foundation_NSEnumerator",
        feature = "Foundation_NSError"
    ))]
    #[objc2::method(
        sel = "enumeratorForContactFetchRequest:error:",
        managed = "Other",
        throws
    )]
    pub unsafe fn enumeratorForContactFetchRequest_error(
        &self,
        request: &CNContactFetchRequest,
    ) -> Result<Id<CNFetchResult<NSEnumerator<CNContact>>>, Id<NSError>>;

    #[cfg(all(
        feature = "Contacts_CNChangeHistoryEvent",
        feature = "Contacts_CNChangeHistoryFetchRequest",
        feature = "Contacts_CNFetchResult",
        feature = "Foundation_NSEnumerator",
        feature = "Foundation_NSError"
    ))]
    #[objc2::method(
        sel = "enumeratorForChangeHistoryFetchRequest:error:",
        managed = "Other",
        throws
    )]
    pub unsafe fn enumeratorForChangeHistoryFetchRequest_error(
        &self,
        request: &CNChangeHistoryFetchRequest,
    ) -> Result<Id<CNFetchResult<NSEnumerator<CNChangeHistoryEvent>>>, Id<NSError>>;

    #[cfg(all(
        feature = "Contacts_CNContact",
        feature = "Contacts_CNContactFetchRequest",
        feature = "Foundation_NSError"
    ))]
    #[objc2::method(sel = "enumerateContactsWithFetchRequest:error:usingBlock:")]
    pub unsafe fn enumerateContactsWithFetchRequest_error_usingBlock(
        &self,
        fetch_request: &CNContactFetchRequest,
        error: Option<&mut Option<Id<NSError>>>,
        block: &Block<(NonNull<CNContact>, NonNull<Bool>), ()>,
    ) -> bool;

    #[cfg(all(
        feature = "Contacts_CNGroup",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError",
        feature = "Foundation_NSPredicate"
    ))]
    #[objc2::method(sel = "groupsMatchingPredicate:error:", managed = "Other", throws)]
    pub unsafe fn groupsMatchingPredicate_error(
        &self,
        predicate: Option<&NSPredicate>,
    ) -> Result<Id<NSArray<CNGroup>>, Id<NSError>>;

    #[cfg(all(
        feature = "Contacts_CNContainer",
        feature = "Foundation_NSArray",
        feature = "Foundation_NSError",
        feature = "Foundation_NSPredicate"
    ))]
    #[objc2::method(sel = "containersMatchingPredicate:error:", managed = "Other", throws)]
    pub unsafe fn containersMatchingPredicate_error(
        &self,
        predicate: Option<&NSPredicate>,
    ) -> Result<Id<NSArray<CNContainer>>, Id<NSError>>;

    #[cfg(all(feature = "Contacts_CNSaveRequest", feature = "Foundation_NSError"))]
    #[objc2::method(sel = "executeSaveRequest:error:", throws)]
    pub unsafe fn executeSaveRequest_error(
        &self,
        save_request: &CNSaveRequest,
    ) -> Result<(), Id<NSError>>;

    #[cfg(feature = "Foundation_NSData")]
    #[objc2::method(sel = "currentHistoryToken", managed = "Other")]
    pub unsafe fn currentHistoryToken(&self) -> Option<Id<NSData>>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "defaultContainerIdentifier", managed = "Other")]
    pub unsafe fn defaultContainerIdentifier(&self) -> Option<Id<NSString>>;
}

extern_static!(CNContactStoreDidChangeNotification: &'static NSString);
