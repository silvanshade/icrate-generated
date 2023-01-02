//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSDictionary<
        KeyType: Message = Object,
        ObjectType: Message = Object,
        KeyTypeOwnership: Ownership = Shared,
        ObjectTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (KeyType, KeyTypeOwnership)>,
        _inner1: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > ClassType for NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method(count)]
        pub fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub fn objectForKey(&self, aKey: &KeyType) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[method_id(@__retain_semantics Other keyEnumerator)]
        pub unsafe fn keyEnumerator(&self) -> Id<NSEnumerator<KeyType>, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithObjects:forKeys:count:)]
        pub unsafe fn initWithObjects_forKeys_count(
            this: Option<Allocated<Self>>,
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<Object>,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSExtendedDictionary
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other allKeys)]
        pub fn allKeys(&self) -> Id<NSArray<KeyType>, Shared>;

        #[method_id(@__retain_semantics Other allKeysForObject:)]
        pub unsafe fn allKeysForObject(
            &self,
            anObject: &ObjectType,
        ) -> Id<NSArray<KeyType>, Shared>;

        #[method_id(@__retain_semantics Other allValues)]
        pub fn allValues(&self) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionInStringsFileFormat)]
        pub unsafe fn descriptionInStringsFileFormat(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithLocale:indent:)]
        pub unsafe fn descriptionWithLocale_indent(
            &self,
            locale: Option<&Object>,
            level: NSUInteger,
        ) -> Id<NSString, Shared>;

        #[method(isEqualToDictionary:)]
        pub unsafe fn isEqualToDictionary(
            &self,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
        ) -> bool;

        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other objectsForKeys:notFoundMarker:)]
        pub unsafe fn objectsForKeys_notFoundMarker(
            &self,
            keys: &NSArray<KeyType>,
            marker: &ObjectType,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method(writeToURL:error:_)]
        pub unsafe fn writeToURL_error(&self, url: &NSURL) -> Result<(), Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other keysSortedByValueUsingSelector:)]
        pub unsafe fn keysSortedByValueUsingSelector(
            &self,
            comparator: Sel,
        ) -> Id<NSArray<KeyType>, Shared>;

        #[method(getObjects:andKeys:count:)]
        pub unsafe fn getObjects_andKeys_count(
            &self,
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<KeyType>,
            count: NSUInteger,
        );

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(
            &self,
            key: &KeyType,
        ) -> Option<Id<ObjectType, ObjectTypeOwnership>>;

        #[method(enumerateKeysAndObjectsUsingBlock:)]
        pub unsafe fn enumerateKeysAndObjectsUsingBlock(
            &self,
            block: &Block<(NonNull<KeyType>, NonNull<ObjectType>, NonNull<Bool>), ()>,
        );

        #[method(enumerateKeysAndObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateKeysAndObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &Block<(NonNull<KeyType>, NonNull<ObjectType>, NonNull<Bool>), ()>,
        );

        #[method_id(@__retain_semantics Other keysSortedByValueUsingComparator:)]
        pub unsafe fn keysSortedByValueUsingComparator(
            &self,
            cmptr: NSComparator,
        ) -> Id<NSArray<KeyType>, Shared>;

        #[method_id(@__retain_semantics Other keysSortedByValueWithOptions:usingComparator:)]
        pub unsafe fn keysSortedByValueWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        ) -> Id<NSArray<KeyType>, Shared>;

        #[method_id(@__retain_semantics Other keysOfEntriesPassingTest:)]
        pub unsafe fn keysOfEntriesPassingTest(
            &self,
            predicate: &Block<(NonNull<KeyType>, NonNull<ObjectType>, NonNull<Bool>), Bool>,
        ) -> Id<NSSet<KeyType>, Shared>;

        #[method_id(@__retain_semantics Other keysOfEntriesWithOptions:passingTest:)]
        pub unsafe fn keysOfEntriesWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<KeyType>, NonNull<ObjectType>, NonNull<Bool>), Bool>,
        ) -> Id<NSSet<KeyType>, Shared>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method(getObjects:andKeys:)]
        pub unsafe fn getObjects_andKeys(
            &self,
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<KeyType>,
        );

        #[method_id(@__retain_semantics Other dictionaryWithContentsOfFile:)]
        pub unsafe fn dictionaryWithContentsOfFile(
            path: &NSString,
        ) -> Option<Id<NSDictionary<KeyType, ObjectType>, Shared>>;

        #[method_id(@__retain_semantics Other dictionaryWithContentsOfURL:)]
        pub unsafe fn dictionaryWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Id<NSDictionary<KeyType, ObjectType>, Shared>>;

        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            useAuxiliaryFile: bool,
        ) -> bool;

        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;
    }
);

extern_methods!(
    /// NSDictionaryCreation
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dictionaryWithObject:forKey:)]
        pub unsafe fn dictionaryWithObject_forKey(
            object: &ObjectType,
            key: &Object,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dictionaryWithObjects:forKeys:count:)]
        pub unsafe fn dictionaryWithObjects_forKeys_count(
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<Object>,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dictionaryWithDictionary:)]
        pub unsafe fn dictionaryWithDictionary(
            dict: &NSDictionary<KeyType, ObjectType>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dictionaryWithObjects:forKeys:)]
        pub unsafe fn dictionaryWithObjects_forKeys(
            objects: &NSArray<ObjectType>,
            keys: &NSArray<Object>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Option<Allocated<Self>>,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithDictionary:copyItems:)]
        pub unsafe fn initWithDictionary_copyItems(
            this: Option<Allocated<Self>>,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithObjects:forKeys:)]
        pub unsafe fn initWithObjects_forKeys(
            this: Option<Allocated<Self>>,
            objects: &NSArray<ObjectType>,
            keys: &NSArray<Object>,
        ) -> Id<Self, Shared>;
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSMutableDictionary<
        KeyType: Message = Object,
        ObjectType: Message = Object,
        KeyTypeOwnership: Ownership = Shared,
        ObjectTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (KeyType, KeyTypeOwnership)>,
        _inner1: PhantomData<*mut (ObjectType, ObjectTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > ClassType
        for NSMutableDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[inherits(NSObject)]
        type Super = NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>;
    }
);

extern_methods!(
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSMutableDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method(removeObjectForKey:)]
        pub fn removeObjectForKey(&mut self, aKey: &KeyType);

        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(&self, anObject: &ObjectType, aKey: &Object);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Option<Allocated<Self>>,
            numItems: NSUInteger,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Owned>>;
    }
);

extern_methods!(
    /// NSExtendedMutableDictionary
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSMutableDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method(addEntriesFromDictionary:)]
        pub unsafe fn addEntriesFromDictionary(
            &self,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
        );

        #[method(removeAllObjects)]
        pub fn removeAllObjects(&mut self);

        #[method(removeObjectsForKeys:)]
        pub unsafe fn removeObjectsForKeys(&self, keyArray: &NSArray<KeyType>);

        #[method(setDictionary:)]
        pub fn setDictionary(&mut self, otherDictionary: &NSDictionary<KeyType, ObjectType>);

        #[method(setObject:forKeyedSubscript:)]
        pub unsafe fn setObject_forKeyedSubscript(&self, obj: Option<&ObjectType>, key: &Object);
    }
);

extern_methods!(
    /// NSMutableDictionaryCreation
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSMutableDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other dictionaryWithCapacity:)]
        pub unsafe fn dictionaryWithCapacity(numItems: NSUInteger) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other dictionaryWithContentsOfFile:)]
        pub unsafe fn dictionaryWithContentsOfFile(
            path: &NSString,
        ) -> Option<Id<NSMutableDictionary<KeyType, ObjectType>, Owned>>;

        #[method_id(@__retain_semantics Other dictionaryWithContentsOfURL:)]
        pub unsafe fn dictionaryWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Id<NSMutableDictionary<KeyType, ObjectType>, Owned>>;
    }
);

extern_methods!(
    /// NSSharedKeySetDictionary
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other sharedKeySetForKeys:)]
        pub unsafe fn sharedKeySetForKeys(keys: &NSArray<Object>) -> Id<Object, Shared>;
    }
);

extern_methods!(
    /// NSSharedKeySetDictionary
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSMutableDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other dictionaryWithSharedKeySet:)]
        pub unsafe fn dictionaryWithSharedKeySet(
            keyset: &Object,
        ) -> Id<NSMutableDictionary<KeyType, ObjectType>, Owned>;
    }
);

extern_methods!(
    /// NSGenericFastEnumeraiton
    unsafe impl<K: Message, V: Message, KOwnership: Ownership, VOwnership: Ownership>
        NSDictionary<K, V, KOwnership, VOwnership>
    {
        #[method(countByEnumeratingWithState:objects:count:)]
        pub unsafe fn countByEnumeratingWithState_objects_count(
            &self,
            state: NonNull<NSFastEnumerationState>,
            buffer: NonNull<*mut K>,
            len: NSUInteger,
        ) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDictionary`
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSMutableDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Init initWithObjects:forKeys:count:)]
        pub unsafe fn initWithObjects_forKeys_count(
            this: Option<Allocated<Self>>,
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<Object>,
            cnt: NSUInteger,
        ) -> Id<Self, Owned>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDictionary`
    ///
    /// NSDictionaryCreation
    unsafe impl<
            KeyType: Message,
            ObjectType: Message,
            KeyTypeOwnership: Ownership,
            ObjectTypeOwnership: Ownership,
        > NSMutableDictionary<KeyType, ObjectType, KeyTypeOwnership, ObjectTypeOwnership>
    {
        #[method_id(@__retain_semantics Other dictionary)]
        pub unsafe fn dictionary() -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other dictionaryWithObject:forKey:)]
        pub unsafe fn dictionaryWithObject_forKey(
            object: &ObjectType,
            key: &Object,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other dictionaryWithObjects:forKeys:count:)]
        pub unsafe fn dictionaryWithObjects_forKeys_count(
            objects: *mut NonNull<ObjectType>,
            keys: *mut NonNull<Object>,
            cnt: NSUInteger,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other dictionaryWithDictionary:)]
        pub unsafe fn dictionaryWithDictionary(
            dict: &NSDictionary<KeyType, ObjectType>,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Other dictionaryWithObjects:forKeys:)]
        pub unsafe fn dictionaryWithObjects_forKeys(
            objects: &NSArray<ObjectType>,
            keys: &NSArray<Object>,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithDictionary:)]
        pub unsafe fn initWithDictionary(
            this: Option<Allocated<Self>>,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithDictionary:copyItems:)]
        pub unsafe fn initWithDictionary_copyItems(
            this: Option<Allocated<Self>>,
            otherDictionary: &NSDictionary<KeyType, ObjectType>,
            flag: bool,
        ) -> Id<Self, Owned>;

        #[method_id(@__retain_semantics Init initWithObjects:forKeys:)]
        pub unsafe fn initWithObjects_forKeys(
            this: Option<Allocated<Self>>,
            objects: &NSArray<ObjectType>,
            keys: &NSArray<Object>,
        ) -> Id<Self, Owned>;
    }
);
