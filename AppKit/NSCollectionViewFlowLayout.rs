//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum NSCollectionViewScrollDirection {
    NSCollectionViewScrollDirectionVertical = 0,
    NSCollectionViewScrollDirectionHorizontal = 1,
}

extern_static!(
    NSCollectionElementKindSectionHeader: &'static NSCollectionViewSupplementaryElementKind
);

extern_static!(
    NSCollectionElementKindSectionFooter: &'static NSCollectionViewSupplementaryElementKind
);

#[objc2::interface(
    unsafe super = NSCollectionViewLayoutInvalidationContext,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSCollectionViewFlowLayoutInvalidationContext")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSCollectionViewFlowLayoutInvalidationContext;
}

#[cfg(feature = "AppKit_NSCollectionViewFlowLayoutInvalidationContext")]
unsafe impl NSObjectProtocol for NSCollectionViewFlowLayoutInvalidationContext {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSCollectionViewFlowLayoutInvalidationContext")]
    pub type NSCollectionViewFlowLayoutInvalidationContext;

    #[objc2::method(sel = "invalidateFlowLayoutDelegateMetrics")]
    pub unsafe fn invalidateFlowLayoutDelegateMetrics(&self) -> bool;

    #[objc2::method(sel = "setInvalidateFlowLayoutDelegateMetrics:")]
    pub unsafe fn setInvalidateFlowLayoutDelegateMetrics(
        &self,
        invalidate_flow_layout_delegate_metrics: bool,
    );

    #[objc2::method(sel = "invalidateFlowLayoutAttributes")]
    pub unsafe fn invalidateFlowLayoutAttributes(&self) -> bool;

    #[objc2::method(sel = "setInvalidateFlowLayoutAttributes:")]
    pub unsafe fn setInvalidateFlowLayoutAttributes(&self, invalidate_flow_layout_attributes: bool);
}

#[objc2::protocol]
pub unsafe trait NSCollectionViewDelegateFlowLayout: NSCollectionViewDelegate {
    #[cfg(all(
        feature = "AppKit_NSCollectionView",
        feature = "AppKit_NSCollectionViewLayout",
        feature = "Foundation_NSIndexPath"
    ))]
    #[objc2::method(optional, sel = "collectionView:layout:sizeForItemAtIndexPath:")]
    unsafe fn collectionView_layout_sizeForItemAtIndexPath(
        &self,
        collection_view: &NSCollectionView,
        collection_view_layout: &NSCollectionViewLayout,
        index_path: &NSIndexPath,
    ) -> NSSize;

    #[cfg(all(
        feature = "AppKit_NSCollectionView",
        feature = "AppKit_NSCollectionViewLayout"
    ))]
    #[objc2::method(optional, sel = "collectionView:layout:insetForSectionAtIndex:")]
    unsafe fn collectionView_layout_insetForSectionAtIndex(
        &self,
        collection_view: &NSCollectionView,
        collection_view_layout: &NSCollectionViewLayout,
        section: NSInteger,
    ) -> NSEdgeInsets;

    #[cfg(all(
        feature = "AppKit_NSCollectionView",
        feature = "AppKit_NSCollectionViewLayout"
    ))]
    #[objc2::method(
        optional,
        sel = "collectionView:layout:minimumLineSpacingForSectionAtIndex:"
    )]
    unsafe fn collectionView_layout_minimumLineSpacingForSectionAtIndex(
        &self,
        collection_view: &NSCollectionView,
        collection_view_layout: &NSCollectionViewLayout,
        section: NSInteger,
    ) -> CGFloat;

    #[cfg(all(
        feature = "AppKit_NSCollectionView",
        feature = "AppKit_NSCollectionViewLayout"
    ))]
    #[objc2::method(
        optional,
        sel = "collectionView:layout:minimumInteritemSpacingForSectionAtIndex:"
    )]
    unsafe fn collectionView_layout_minimumInteritemSpacingForSectionAtIndex(
        &self,
        collection_view: &NSCollectionView,
        collection_view_layout: &NSCollectionViewLayout,
        section: NSInteger,
    ) -> CGFloat;

    #[cfg(all(
        feature = "AppKit_NSCollectionView",
        feature = "AppKit_NSCollectionViewLayout"
    ))]
    #[objc2::method(
        optional,
        sel = "collectionView:layout:referenceSizeForHeaderInSection:"
    )]
    unsafe fn collectionView_layout_referenceSizeForHeaderInSection(
        &self,
        collection_view: &NSCollectionView,
        collection_view_layout: &NSCollectionViewLayout,
        section: NSInteger,
    ) -> NSSize;

    #[cfg(all(
        feature = "AppKit_NSCollectionView",
        feature = "AppKit_NSCollectionViewLayout"
    ))]
    #[objc2::method(
        optional,
        sel = "collectionView:layout:referenceSizeForFooterInSection:"
    )]
    unsafe fn collectionView_layout_referenceSizeForFooterInSection(
        &self,
        collection_view: &NSCollectionView,
        collection_view_layout: &NSCollectionViewLayout,
        section: NSInteger,
    ) -> NSSize;
}

#[objc2::interface(
    unsafe super = NSCollectionViewLayout,
    unsafe inherits = [
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSCollectionViewFlowLayout")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type NSCollectionViewFlowLayout;
}

#[cfg(feature = "AppKit_NSCollectionViewFlowLayout")]
unsafe impl NSCoding for NSCollectionViewFlowLayout {}

#[cfg(feature = "AppKit_NSCollectionViewFlowLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewFlowLayout {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "AppKit_NSCollectionViewFlowLayout")]
    pub type NSCollectionViewFlowLayout;

    #[objc2::method(sel = "minimumLineSpacing")]
    pub unsafe fn minimumLineSpacing(&self) -> CGFloat;

    #[objc2::method(sel = "setMinimumLineSpacing:")]
    pub unsafe fn setMinimumLineSpacing(&self, minimum_line_spacing: CGFloat);

    #[objc2::method(sel = "minimumInteritemSpacing")]
    pub unsafe fn minimumInteritemSpacing(&self) -> CGFloat;

    #[objc2::method(sel = "setMinimumInteritemSpacing:")]
    pub unsafe fn setMinimumInteritemSpacing(&self, minimum_interitem_spacing: CGFloat);

    #[objc2::method(sel = "itemSize")]
    pub unsafe fn itemSize(&self) -> NSSize;

    #[objc2::method(sel = "setItemSize:")]
    pub unsafe fn setItemSize(&self, item_size: NSSize);

    #[objc2::method(sel = "estimatedItemSize")]
    pub unsafe fn estimatedItemSize(&self) -> NSSize;

    #[objc2::method(sel = "setEstimatedItemSize:")]
    pub unsafe fn setEstimatedItemSize(&self, estimated_item_size: NSSize);

    #[objc2::method(sel = "scrollDirection")]
    pub unsafe fn scrollDirection(&self) -> NSCollectionViewScrollDirection;

    #[objc2::method(sel = "setScrollDirection:")]
    pub unsafe fn setScrollDirection(&self, scroll_direction: NSCollectionViewScrollDirection);

    #[objc2::method(sel = "headerReferenceSize")]
    pub unsafe fn headerReferenceSize(&self) -> NSSize;

    #[objc2::method(sel = "setHeaderReferenceSize:")]
    pub unsafe fn setHeaderReferenceSize(&self, header_reference_size: NSSize);

    #[objc2::method(sel = "footerReferenceSize")]
    pub unsafe fn footerReferenceSize(&self) -> NSSize;

    #[objc2::method(sel = "setFooterReferenceSize:")]
    pub unsafe fn setFooterReferenceSize(&self, footer_reference_size: NSSize);

    #[objc2::method(sel = "sectionInset")]
    pub unsafe fn sectionInset(&self) -> NSEdgeInsets;

    #[objc2::method(sel = "setSectionInset:")]
    pub unsafe fn setSectionInset(&self, section_inset: NSEdgeInsets);

    #[objc2::method(sel = "sectionHeadersPinToVisibleBounds")]
    pub unsafe fn sectionHeadersPinToVisibleBounds(&self) -> bool;

    #[objc2::method(sel = "setSectionHeadersPinToVisibleBounds:")]
    pub unsafe fn setSectionHeadersPinToVisibleBounds(
        &self,
        section_headers_pin_to_visible_bounds: bool,
    );

    #[objc2::method(sel = "sectionFootersPinToVisibleBounds")]
    pub unsafe fn sectionFootersPinToVisibleBounds(&self) -> bool;

    #[objc2::method(sel = "setSectionFootersPinToVisibleBounds:")]
    pub unsafe fn setSectionFootersPinToVisibleBounds(
        &self,
        section_footers_pin_to_visible_bounds: bool,
    );

    #[objc2::method(sel = "sectionAtIndexIsCollapsed:")]
    pub unsafe fn sectionAtIndexIsCollapsed(&self, section_index: NSUInteger) -> bool;

    #[objc2::method(sel = "collapseSectionAtIndex:")]
    pub unsafe fn collapseSectionAtIndex(&self, section_index: NSUInteger);

    #[objc2::method(sel = "expandSectionAtIndex:")]
    pub unsafe fn expandSectionAtIndex(&self, section_index: NSUInteger);
}
