//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation;
use crate::Metal;

extern_struct!(
    pub struct MTLPackedFloat4x3 {
        pub columns: [Metal::MTLPackedFloat3; 4],
    }
);

extern_struct!(
    pub struct MTLAxisAlignedBoundingBox {
        pub min: Metal::MTLPackedFloat3,
        pub max: Metal::MTLPackedFloat3,
    }
);
