//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataReadingOptions {
        NSDataReadingMappedIfSafe = 1 << 0,
        NSDataReadingUncached = 1 << 1,
        NSDataReadingMappedAlways = 1 << 3,
        NSDataReadingMapped = NSDataReadingMappedIfSafe,
        NSMappedRead = NSDataReadingMapped,
        NSUncachedRead = NSDataReadingUncached,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataWritingOptions {
        NSDataWritingAtomic = 1 << 0,
        NSDataWritingWithoutOverwriting = 1 << 1,
        NSDataWritingFileProtectionNone = 0x10000000,
        NSDataWritingFileProtectionComplete = 0x20000000,
        NSDataWritingFileProtectionCompleteUnlessOpen = 0x30000000,
        NSDataWritingFileProtectionCompleteUntilFirstUserAuthentication = 0x40000000,
        NSDataWritingFileProtectionMask = 0xf0000000,
        NSAtomicWrite = NSDataWritingAtomic,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataSearchOptions {
        NSDataSearchBackwards = 1 << 0,
        NSDataSearchAnchored = 1 << 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataBase64EncodingOptions {
        NSDataBase64Encoding64CharacterLineLength = 1 << 0,
        NSDataBase64Encoding76CharacterLineLength = 1 << 1,
        NSDataBase64EncodingEndLineWithCarriageReturn = 1 << 4,
        NSDataBase64EncodingEndLineWithLineFeed = 1 << 5,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataBase64DecodingOptions {
        NSDataBase64DecodingIgnoreUnknownCharacters = 1 << 0,
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSData;

    unsafe impl ClassType for NSData {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[method(length)]
        pub fn length(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// NSExtendedData
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<Foundation::NSString, Shared>;

        #[method(getBytes:length:)]
        pub unsafe fn getBytes_length(&self, buffer: NonNull<c_void>, length: NSUInteger);

        #[method(getBytes:range:)]
        pub unsafe fn getBytes_range(&self, buffer: NonNull<c_void>, range: Foundation::NSRange);

        #[method(isEqualToData:)]
        pub unsafe fn isEqualToData(&self, other: &Foundation::NSData) -> bool;

        #[method_id(@__retain_semantics Other subdataWithRange:)]
        pub unsafe fn subdataWithRange(
            &self,
            range: Foundation::NSRange,
        ) -> Id<Foundation::NSData, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &Foundation::NSString,
            useAuxiliaryFile: bool,
        ) -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(
            &self,
            url: &Foundation::NSURL,
            atomically: bool,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(writeToFile:options:error:_)]
        pub unsafe fn writeToFile_options_error(
            &self,
            path: &Foundation::NSString,
            writeOptionsMask: Foundation::NSDataWritingOptions,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(writeToURL:options:error:_)]
        pub unsafe fn writeToURL_options_error(
            &self,
            url: &Foundation::NSURL,
            writeOptionsMask: Foundation::NSDataWritingOptions,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[method(rangeOfData:options:range:)]
        pub unsafe fn rangeOfData_options_range(
            &self,
            dataToFind: &Foundation::NSData,
            mask: Foundation::NSDataSearchOptions,
            searchRange: Foundation::NSRange,
        ) -> Foundation::NSRange;

        #[method(enumerateByteRangesUsingBlock:)]
        pub unsafe fn enumerateByteRangesUsingBlock(
            &self,
            block: &Block<(NonNull<c_void>, Foundation::NSRange, NonNull<Bool>), ()>,
        );
    }
);

extern_methods!(
    /// NSDataCreation
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:options:error:_)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &Foundation::NSString,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:options:error:_)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &Foundation::NSURL,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &Foundation::NSURL) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            this: Option<Allocated<Self>>,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: Option<&Block<(NonNull<c_void>, NSUInteger), ()>>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:options:error:_)]
        pub unsafe fn initWithContentsOfFile_options_error(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub fn initWithData(
            this: Option<Allocated<Self>>,
            data: &Foundation::NSData,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dataWithData:)]
        pub fn dataWithData(data: &Foundation::NSData) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSDataBase64Encoding
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            this: Option<Allocated<Self>>,
            base64String: &Foundation::NSString,
            options: Foundation::NSDataBase64DecodingOptions,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other base64EncodedStringWithOptions:)]
        pub unsafe fn base64EncodedStringWithOptions(
            &self,
            options: Foundation::NSDataBase64EncodingOptions,
        ) -> Id<Foundation::NSString, Shared>;

        #[method_id(@__retain_semantics Init initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            this: Option<Allocated<Self>>,
            base64Data: &Foundation::NSData,
            options: Foundation::NSDataBase64DecodingOptions,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other base64EncodedDataWithOptions:)]
        pub unsafe fn base64EncodedDataWithOptions(
            &self,
            options: Foundation::NSDataBase64EncodingOptions,
        ) -> Id<Foundation::NSData, Shared>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDataCompressionAlgorithm {
        NSDataCompressionAlgorithmLZFSE = 0,
        NSDataCompressionAlgorithmLZ4 = 1,
        NSDataCompressionAlgorithmLZMA = 2,
        NSDataCompressionAlgorithmZlib = 3,
    }
);

extern_methods!(
    /// NSDataCompression
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other decompressedDataUsingAlgorithm:error:_)]
        pub unsafe fn decompressedDataUsingAlgorithm_error(
            &self,
            algorithm: Foundation::NSDataCompressionAlgorithm,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other compressedDataUsingAlgorithm:error:_)]
        pub unsafe fn compressedDataUsingAlgorithm_error(
            &self,
            algorithm: Foundation::NSDataCompressionAlgorithm,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[method(getBytes:)]
        pub unsafe fn getBytes(&self, buffer: NonNull<c_void>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfMappedFile:)]
        pub unsafe fn dataWithContentsOfMappedFile(
            path: &Foundation::NSString,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            this: Option<Allocated<Self>>,
            base64String: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other base64Encoding)]
        pub unsafe fn base64Encoding(&self) -> Id<Foundation::NSString, Shared>;
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSMutableData;

    unsafe impl ClassType for NSMutableData {
        #[inherits(NSObject)]
        type Super = NSData;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[method(setLength:)]
        pub fn setLength(&mut self, length: NSUInteger);
    }
);

extern_methods!(
    /// NSExtendedMutableData
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[method(appendBytes:length:)]
        pub unsafe fn appendBytes_length(&self, bytes: NonNull<c_void>, length: NSUInteger);

        #[cfg(feature = "Foundation_NSData")]
        #[method(appendData:)]
        pub unsafe fn appendData(&self, other: &Foundation::NSData);

        #[method(increaseLengthBy:)]
        pub unsafe fn increaseLengthBy(&self, extraLength: NSUInteger);

        #[method(replaceBytesInRange:withBytes:)]
        pub unsafe fn replaceBytesInRange_withBytes(
            &self,
            range: Foundation::NSRange,
            bytes: NonNull<c_void>,
        );

        #[method(resetBytesInRange:)]
        pub unsafe fn resetBytesInRange(&self, range: Foundation::NSRange);

        #[cfg(feature = "Foundation_NSData")]
        #[method(setData:)]
        pub unsafe fn setData(&self, data: &Foundation::NSData);

        #[method(replaceBytesInRange:withBytes:length:)]
        pub unsafe fn replaceBytesInRange_withBytes_length(
            &self,
            range: Foundation::NSRange,
            replacementBytes: *mut c_void,
            replacementLength: NSUInteger,
        );
    }
);

extern_methods!(
    /// NSMutableDataCreation
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Other dataWithCapacity:)]
        pub fn dataWithCapacity(aNumItems: NSUInteger) -> Option<Id<Self, Owned>>;

        #[method_id(@__retain_semantics Other dataWithLength:)]
        pub unsafe fn dataWithLength(length: NSUInteger) -> Option<Id<Self, Owned>>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub fn initWithCapacity(
            this: Option<Allocated<Self>>,
            capacity: NSUInteger,
        ) -> Option<Id<Self, Owned>>;

        #[method_id(@__retain_semantics Init initWithLength:)]
        pub unsafe fn initWithLength(
            this: Option<Allocated<Self>>,
            length: NSUInteger,
        ) -> Option<Id<Self, Owned>>;
    }
);

extern_methods!(
    /// NSMutableDataCompression
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[cfg(feature = "Foundation_NSError")]
        #[method(decompressUsingAlgorithm:error:_)]
        pub unsafe fn decompressUsingAlgorithm_error(
            &self,
            algorithm: Foundation::NSDataCompressionAlgorithm,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(compressUsingAlgorithm:error:_)]
        pub unsafe fn compressUsingAlgorithm_error(
            &self,
            algorithm: Foundation::NSDataCompressionAlgorithm,
        ) -> Result<(), Id<Foundation::NSError, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPurgeableData;

    unsafe impl ClassType for NSPurgeableData {
        #[inherits(NSData, NSObject)]
        type Super = NSMutableData;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSPurgeableData")]
    unsafe impl NSPurgeableData {}
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataCreation
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self, Owned>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:options:error:_)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &Foundation::NSString,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Owned>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:options:error:_)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &Foundation::NSURL,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Owned>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Owned>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &Foundation::NSURL) -> Option<Id<Self, Owned>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            this: Option<Allocated<Self>>,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: Option<&Block<(NonNull<c_void>, NSUInteger), ()>>,
        ) -> Id<Self, Owned>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:options:error:_)]
        pub unsafe fn initWithContentsOfFile_options_error(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Owned>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Owned>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Owned>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Option<Id<Self, Owned>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: &Foundation::NSData,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other dataWithData:)]
        pub fn dataWithData(data: &Foundation::NSData) -> Id<Self, Owned>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataBase64Encoding
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            this: Option<Allocated<Self>>,
            base64String: &Foundation::NSString,
            options: Foundation::NSDataBase64DecodingOptions,
        ) -> Option<Id<Self, Owned>>;

        #[method_id(@__retain_semantics Init initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            this: Option<Allocated<Self>>,
            base64Data: &Foundation::NSData,
            options: Foundation::NSDataBase64DecodingOptions,
        ) -> Option<Id<Self, Owned>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Owned>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            this: Option<Allocated<Self>>,
            base64String: &Foundation::NSString,
        ) -> Option<Id<Self, Owned>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSMutableData`
    ///
    /// NSMutableDataCreation
    #[cfg(feature = "Foundation_NSPurgeableData")]
    unsafe impl NSPurgeableData {
        #[method_id(@__retain_semantics Other dataWithCapacity:)]
        pub unsafe fn dataWithCapacity(aNumItems: NSUInteger) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other dataWithLength:)]
        pub unsafe fn dataWithLength(length: NSUInteger) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Option<Allocated<Self>>,
            capacity: NSUInteger,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithLength:)]
        pub unsafe fn initWithLength(
            this: Option<Allocated<Self>>,
            length: NSUInteger,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataCreation
    #[cfg(feature = "Foundation_NSPurgeableData")]
    unsafe impl NSPurgeableData {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:options:error:_)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &Foundation::NSString,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:options:error:_)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &Foundation::NSURL,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &Foundation::NSURL) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            this: Option<Allocated<Self>>,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            this: Option<Allocated<Self>>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: Option<&Block<(NonNull<c_void>, NSUInteger), ()>>,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:options:error:_)]
        pub unsafe fn initWithContentsOfFile_options_error(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
            readOptionsMask: Foundation::NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<Foundation::NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &Foundation::NSURL,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: &Foundation::NSData,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dataWithData:)]
        pub unsafe fn dataWithData(data: &Foundation::NSData) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataBase64Encoding
    #[cfg(feature = "Foundation_NSPurgeableData")]
    unsafe impl NSPurgeableData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            this: Option<Allocated<Self>>,
            base64String: &Foundation::NSString,
            options: Foundation::NSDataBase64DecodingOptions,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            this: Option<Allocated<Self>>,
            base64Data: &Foundation::NSData,
            options: Foundation::NSDataBase64DecodingOptions,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSPurgeableData")]
    unsafe impl NSPurgeableData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            this: Option<Allocated<Self>>,
            path: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            this: Option<Allocated<Self>>,
            base64String: &Foundation::NSString,
        ) -> Option<Id<Self, Shared>>;
    }
);
