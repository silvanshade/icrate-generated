//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTypesetter;

    unsafe impl ClassType for NSTypesetter {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTypesetter")]
    unsafe impl NSTypesetter {
        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;

        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, usesFontLeading: bool);

        #[method(typesetterBehavior)]
        pub unsafe fn typesetterBehavior(&self) -> AppKit::NSTypesetterBehavior;

        #[method(setTypesetterBehavior:)]
        pub unsafe fn setTypesetterBehavior(
            &self,
            typesetterBehavior: AppKit::NSTypesetterBehavior,
        );

        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;

        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenationFactor: c_float);

        #[method(lineFragmentPadding)]
        pub unsafe fn lineFragmentPadding(&self) -> Foundation::CGFloat;

        #[method(setLineFragmentPadding:)]
        pub unsafe fn setLineFragmentPadding(&self, lineFragmentPadding: Foundation::CGFloat);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other substituteFontForFont:)]
        pub unsafe fn substituteFontForFont(
            &self,
            originalFont: &AppKit::NSFont,
        ) -> Id<AppKit::NSFont, Shared>;

        #[cfg(feature = "AppKit_NSTextTab")]
        #[method_id(@__retain_semantics Other textTabForGlyphLocation:writingDirection:maxLocation:)]
        pub unsafe fn textTabForGlyphLocation_writingDirection_maxLocation(
            &self,
            glyphLocation: Foundation::CGFloat,
            direction: AppKit::NSWritingDirection,
            maxLocation: Foundation::CGFloat,
        ) -> Option<Id<AppKit::NSTextTab, Shared>>;

        #[method(bidiProcessingEnabled)]
        pub unsafe fn bidiProcessingEnabled(&self) -> bool;

        #[method(setBidiProcessingEnabled:)]
        pub unsafe fn setBidiProcessingEnabled(&self, bidiProcessingEnabled: bool);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self)
            -> Option<Id<Foundation::NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(
            &self,
            attributedString: Option<&Foundation::NSAttributedString>,
        );

        #[method(setParagraphGlyphRange:separatorGlyphRange:)]
        pub unsafe fn setParagraphGlyphRange_separatorGlyphRange(
            &self,
            paragraphRange: Foundation::NSRange,
            paragraphSeparatorRange: Foundation::NSRange,
        );

        #[method(paragraphGlyphRange)]
        pub unsafe fn paragraphGlyphRange(&self) -> Foundation::NSRange;

        #[method(paragraphSeparatorGlyphRange)]
        pub unsafe fn paragraphSeparatorGlyphRange(&self) -> Foundation::NSRange;

        #[method(paragraphCharacterRange)]
        pub unsafe fn paragraphCharacterRange(&self) -> Foundation::NSRange;

        #[method(paragraphSeparatorCharacterRange)]
        pub unsafe fn paragraphSeparatorCharacterRange(&self) -> Foundation::NSRange;

        #[method(layoutParagraphAtPoint:)]
        pub unsafe fn layoutParagraphAtPoint(
            &self,
            lineFragmentOrigin: Foundation::NSPointPointer,
        ) -> NSUInteger;

        #[method(beginParagraph)]
        pub unsafe fn beginParagraph(&self);

        #[method(endParagraph)]
        pub unsafe fn endParagraph(&self);

        #[method(beginLineWithGlyphAtIndex:)]
        pub unsafe fn beginLineWithGlyphAtIndex(&self, glyphIndex: NSUInteger);

        #[method(endLineWithGlyphRange:)]
        pub unsafe fn endLineWithGlyphRange(&self, lineGlyphRange: Foundation::NSRange);

        #[method(lineSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn lineSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyphIndex: NSUInteger,
            rect: Foundation::NSRect,
        ) -> Foundation::CGFloat;

        #[method(paragraphSpacingBeforeGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn paragraphSpacingBeforeGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyphIndex: NSUInteger,
            rect: Foundation::NSRect,
        ) -> Foundation::CGFloat;

        #[method(paragraphSpacingAfterGlyphAtIndex:withProposedLineFragmentRect:)]
        pub unsafe fn paragraphSpacingAfterGlyphAtIndex_withProposedLineFragmentRect(
            &self,
            glyphIndex: NSUInteger,
            rect: Foundation::NSRect,
        ) -> Foundation::CGFloat;

        #[method(getLineFragmentRect:usedRect:forParagraphSeparatorGlyphRange:atProposedOrigin:)]
        pub unsafe fn getLineFragmentRect_usedRect_forParagraphSeparatorGlyphRange_atProposedOrigin(
            &self,
            lineFragmentRect: Foundation::NSRectPointer,
            lineFragmentUsedRect: Foundation::NSRectPointer,
            paragraphSeparatorGlyphRange: Foundation::NSRange,
            lineOrigin: Foundation::NSPoint,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other attributesForExtraLineFragment)]
        pub unsafe fn attributesForExtraLineFragment(
            &self,
        ) -> Id<Foundation::NSDictionary<Foundation::NSAttributedStringKey, Object>, Shared>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method_id(@__retain_semantics Other layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Id<AppKit::NSLayoutManager, Shared>>;

        #[cfg(all(feature = "AppKit_NSTextContainer", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textContainers)]
        pub unsafe fn textContainers(
            &self,
        ) -> Option<Id<Foundation::NSArray<AppKit::NSTextContainer>, Shared>>;

        #[cfg(feature = "AppKit_NSTextContainer")]
        #[method_id(@__retain_semantics Other currentTextContainer)]
        pub unsafe fn currentTextContainer(&self) -> Option<Id<AppKit::NSTextContainer, Shared>>;

        #[cfg(feature = "AppKit_NSParagraphStyle")]
        #[method_id(@__retain_semantics Other currentParagraphStyle)]
        pub unsafe fn currentParagraphStyle(&self) -> Option<Id<AppKit::NSParagraphStyle, Shared>>;

        #[method(setHardInvalidation:forGlyphRange:)]
        pub unsafe fn setHardInvalidation_forGlyphRange(
            &self,
            flag: bool,
            glyphRange: Foundation::NSRange,
        );

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(layoutGlyphsInLayoutManager:startingAtGlyphIndex:maxNumberOfLineFragments:nextGlyphIndex:)]
        pub unsafe fn layoutGlyphsInLayoutManager_startingAtGlyphIndex_maxNumberOfLineFragments_nextGlyphIndex(
            &self,
            layoutManager: &AppKit::NSLayoutManager,
            startGlyphIndex: NSUInteger,
            maxNumLines: NSUInteger,
            nextGlyph: NonNull<NSUInteger>,
        );

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(layoutCharactersInRange:forLayoutManager:maximumNumberOfLineFragments:)]
        pub unsafe fn layoutCharactersInRange_forLayoutManager_maximumNumberOfLineFragments(
            &self,
            characterRange: Foundation::NSRange,
            layoutManager: &AppKit::NSLayoutManager,
            maxNumLines: NSUInteger,
        ) -> Foundation::NSRange;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(printingAdjustmentInLayoutManager:forNominallySpacedGlyphRange:packedGlyphs:count:)]
        pub unsafe fn printingAdjustmentInLayoutManager_forNominallySpacedGlyphRange_packedGlyphs_count(
            layoutMgr: &AppKit::NSLayoutManager,
            nominallySpacedGlyphsRange: Foundation::NSRange,
            packedGlyphs: NonNull<c_uchar>,
            packedGlyphsCount: NSUInteger,
        ) -> Foundation::NSSize;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(baselineOffsetInLayoutManager:glyphIndex:)]
        pub unsafe fn baselineOffsetInLayoutManager_glyphIndex(
            &self,
            layoutMgr: &AppKit::NSLayoutManager,
            glyphIndex: NSUInteger,
        ) -> Foundation::CGFloat;

        #[method_id(@__retain_semantics Other sharedSystemTypesetter)]
        pub unsafe fn sharedSystemTypesetter() -> Id<AppKit::NSTypesetter, Shared>;

        #[method_id(@__retain_semantics Other sharedSystemTypesetterForBehavior:)]
        pub unsafe fn sharedSystemTypesetterForBehavior(
            behavior: AppKit::NSTypesetterBehavior,
        ) -> Id<Object, Shared>;

        #[method(defaultTypesetterBehavior)]
        pub unsafe fn defaultTypesetterBehavior() -> AppKit::NSTypesetterBehavior;
    }
);

extern_methods!(
    /// NSLayoutPhaseInterface
    #[cfg(feature = "AppKit_NSTypesetter")]
    unsafe impl NSTypesetter {
        #[method(willSetLineFragmentRect:forGlyphRange:usedRect:baselineOffset:)]
        pub unsafe fn willSetLineFragmentRect_forGlyphRange_usedRect_baselineOffset(
            &self,
            lineRect: Foundation::NSRectPointer,
            glyphRange: Foundation::NSRange,
            usedRect: Foundation::NSRectPointer,
            baselineOffset: NonNull<Foundation::CGFloat>,
        );

        #[method(shouldBreakLineByWordBeforeCharacterAtIndex:)]
        pub unsafe fn shouldBreakLineByWordBeforeCharacterAtIndex(
            &self,
            charIndex: NSUInteger,
        ) -> bool;

        #[method(shouldBreakLineByHyphenatingBeforeCharacterAtIndex:)]
        pub unsafe fn shouldBreakLineByHyphenatingBeforeCharacterAtIndex(
            &self,
            charIndex: NSUInteger,
        ) -> bool;

        #[method(hyphenationFactorForGlyphAtIndex:)]
        pub unsafe fn hyphenationFactorForGlyphAtIndex(&self, glyphIndex: NSUInteger) -> c_float;

        #[method(hyphenCharacterForGlyphAtIndex:)]
        pub unsafe fn hyphenCharacterForGlyphAtIndex(&self, glyphIndex: NSUInteger) -> UTF32Char;

        #[cfg(feature = "AppKit_NSTextContainer")]
        #[method(boundingBoxForControlGlyphAtIndex:forTextContainer:proposedLineFragment:glyphPosition:characterIndex:)]
        pub unsafe fn boundingBoxForControlGlyphAtIndex_forTextContainer_proposedLineFragment_glyphPosition_characterIndex(
            &self,
            glyphIndex: NSUInteger,
            textContainer: &AppKit::NSTextContainer,
            proposedRect: Foundation::NSRect,
            glyphPosition: Foundation::NSPoint,
            charIndex: NSUInteger,
        ) -> Foundation::NSRect;
    }
);

extern_methods!(
    /// NSGlyphStorageInterface
    #[cfg(feature = "AppKit_NSTypesetter")]
    unsafe impl NSTypesetter {
        #[method(characterRangeForGlyphRange:actualGlyphRange:)]
        pub unsafe fn characterRangeForGlyphRange_actualGlyphRange(
            &self,
            glyphRange: Foundation::NSRange,
            actualGlyphRange: Foundation::NSRangePointer,
        ) -> Foundation::NSRange;

        #[method(glyphRangeForCharacterRange:actualCharacterRange:)]
        pub unsafe fn glyphRangeForCharacterRange_actualCharacterRange(
            &self,
            charRange: Foundation::NSRange,
            actualCharRange: Foundation::NSRangePointer,
        ) -> Foundation::NSRange;

        #[method(getLineFragmentRect:usedRect:remainingRect:forStartingGlyphAtIndex:proposedRect:lineSpacing:paragraphSpacingBefore:paragraphSpacingAfter:)]
        pub unsafe fn getLineFragmentRect_usedRect_remainingRect_forStartingGlyphAtIndex_proposedRect_lineSpacing_paragraphSpacingBefore_paragraphSpacingAfter(
            &self,
            lineFragmentRect: Foundation::NSRectPointer,
            lineFragmentUsedRect: Foundation::NSRectPointer,
            remainingRect: Foundation::NSRectPointer,
            startingGlyphIndex: NSUInteger,
            proposedRect: Foundation::NSRect,
            lineSpacing: Foundation::CGFloat,
            paragraphSpacingBefore: Foundation::CGFloat,
            paragraphSpacingAfter: Foundation::CGFloat,
        );

        #[method(setLineFragmentRect:forGlyphRange:usedRect:baselineOffset:)]
        pub unsafe fn setLineFragmentRect_forGlyphRange_usedRect_baselineOffset(
            &self,
            fragmentRect: Foundation::NSRect,
            glyphRange: Foundation::NSRange,
            usedRect: Foundation::NSRect,
            baselineOffset: Foundation::CGFloat,
        );

        #[method(setNotShownAttribute:forGlyphRange:)]
        pub unsafe fn setNotShownAttribute_forGlyphRange(
            &self,
            flag: bool,
            glyphRange: Foundation::NSRange,
        );

        #[method(setDrawsOutsideLineFragment:forGlyphRange:)]
        pub unsafe fn setDrawsOutsideLineFragment_forGlyphRange(
            &self,
            flag: bool,
            glyphRange: Foundation::NSRange,
        );

        #[method(setLocation:withAdvancements:forStartOfGlyphRange:)]
        pub unsafe fn setLocation_withAdvancements_forStartOfGlyphRange(
            &self,
            location: Foundation::NSPoint,
            advancements: *mut Foundation::CGFloat,
            glyphRange: Foundation::NSRange,
        );

        #[method(setAttachmentSize:forGlyphRange:)]
        pub unsafe fn setAttachmentSize_forGlyphRange(
            &self,
            attachmentSize: Foundation::NSSize,
            glyphRange: Foundation::NSRange,
        );

        #[method(setBidiLevels:forGlyphRange:)]
        pub unsafe fn setBidiLevels_forGlyphRange(
            &self,
            levels: *mut u8,
            glyphRange: Foundation::NSRange,
        );
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTypesetterControlCharacterAction {
        NSTypesetterZeroAdvancementAction = 1 << 0,
        NSTypesetterWhitespaceAction = 1 << 1,
        NSTypesetterHorizontalTabAction = 1 << 2,
        NSTypesetterLineBreakAction = 1 << 3,
        NSTypesetterParagraphBreakAction = 1 << 4,
        NSTypesetterContainerBreakAction = 1 << 5,
    }
);

extern_methods!(
    /// NSTypesetter_Deprecated
    #[cfg(feature = "AppKit_NSTypesetter")]
    unsafe impl NSTypesetter {
        #[method(actionForControlCharacterAtIndex:)]
        pub unsafe fn actionForControlCharacterAtIndex(
            &self,
            charIndex: NSUInteger,
        ) -> AppKit::NSTypesetterControlCharacterAction;

        #[method(getGlyphsInRange:glyphs:characterIndexes:glyphInscriptions:elasticBits:bidiLevels:)]
        pub unsafe fn getGlyphsInRange_glyphs_characterIndexes_glyphInscriptions_elasticBits_bidiLevels(
            &self,
            glyphsRange: Foundation::NSRange,
            glyphBuffer: *mut AppKit::NSGlyph,
            charIndexBuffer: *mut NSUInteger,
            inscribeBuffer: *mut AppKit::NSGlyphInscription,
            elasticBuffer: *mut Bool,
            bidiLevelBuffer: *mut c_uchar,
        ) -> NSUInteger;

        #[method(substituteGlyphsInRange:withGlyphs:)]
        pub unsafe fn substituteGlyphsInRange_withGlyphs(
            &self,
            glyphRange: Foundation::NSRange,
            glyphs: *mut AppKit::NSGlyph,
        );

        #[method(insertGlyph:atGlyphIndex:characterIndex:)]
        pub unsafe fn insertGlyph_atGlyphIndex_characterIndex(
            &self,
            glyph: AppKit::NSGlyph,
            glyphIndex: NSUInteger,
            characterIndex: NSUInteger,
        );

        #[method(deleteGlyphsInRange:)]
        pub unsafe fn deleteGlyphsInRange(&self, glyphRange: Foundation::NSRange);
    }
);
