//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::CoreFoundation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(u32)]
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    pub enum NSOpenGLGlobalOption {
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLGOFormatCacheSize = 501,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLGOClearFormatCache = 502,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLGORetainRenderers = 503,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLGOUseBuildCache = 506,
        #[deprecated]
        NSOpenGLGOResetLibrary = 504,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAAllRenderers = 1,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFATripleBuffer = 3,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFADoubleBuffer = 5,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAAuxBuffers = 7,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAColorSize = 8,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAAlphaSize = 11,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFADepthSize = 12,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAStencilSize = 13,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAAccumSize = 14,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAMinimumPolicy = 51,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAMaximumPolicy = 52,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFASampleBuffers = 55,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFASamples = 56,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAAuxDepthStencil = 57,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAColorFloat = 58,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAMultisample = 59,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFASupersample = 60,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFASampleAlpha = 61,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFARendererID = 70,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFANoRecovery = 72,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAAccelerated = 73,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAClosestPolicy = 74,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFABackingStore = 76,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAScreenMask = 84,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAAllowOfflineRenderers = 96,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAAcceleratedCompute = 97,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAOpenGLProfile = 99,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLPFAVirtualScreenCount = 128,
        #[deprecated]
        NSOpenGLPFAStereo = 6,
        #[deprecated]
        NSOpenGLPFAOffScreen = 53,
        #[deprecated]
        NSOpenGLPFAFullScreen = 54,
        #[deprecated]
        NSOpenGLPFASingleRenderer = 71,
        #[deprecated]
        NSOpenGLPFARobust = 75,
        #[deprecated]
        NSOpenGLPFAMPSafe = 78,
        #[deprecated]
        NSOpenGLPFAWindow = 80,
        #[deprecated]
        NSOpenGLPFAMultiScreen = 81,
        #[deprecated]
        NSOpenGLPFACompliant = 83,
        #[deprecated]
        NSOpenGLPFAPixelBuffer = 90,
        #[deprecated]
        NSOpenGLPFARemotePixelBuffer = 91,
    }
);

pub type NSOpenGLPixelFormatAttribute = u32;

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLProfileVersionLegacy = 0x1000,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLProfileVersion3_2Core = 0x3200,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLProfileVersion4_1Core = 0x4100,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    pub enum NSOpenGLContextParameter {
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterSwapInterval = 222,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterSurfaceOrder = 235,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterSurfaceOpacity = 236,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterSurfaceBackingSize = 304,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterReclaimResources = 308,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterCurrentRendererID = 309,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterGPUVertexProcessing = 310,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterGPUFragmentProcessing = 311,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterHasDrawable = 314,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterMPSwapsInFlight = 315,
        #[deprecated]
        NSOpenGLContextParameterSwapRectangle = 200,
        #[deprecated]
        NSOpenGLContextParameterSwapRectangleEnable = 201,
        #[deprecated]
        NSOpenGLContextParameterRasterizationEnable = 221,
        #[deprecated]
        NSOpenGLContextParameterStateValidation = 301,
        #[deprecated]
        NSOpenGLContextParameterSurfaceSurfaceVolatile = 306,
    }
);

extern_static!(
    NSOpenGLCPSwapInterval: NSOpenGLContextParameter = NSOpenGLContextParameterSwapInterval
);

extern_static!(
    NSOpenGLCPSurfaceOrder: NSOpenGLContextParameter = NSOpenGLContextParameterSurfaceOrder
);

extern_static!(
    NSOpenGLCPSurfaceOpacity: NSOpenGLContextParameter = NSOpenGLContextParameterSurfaceOpacity
);

extern_static!(
    NSOpenGLCPSurfaceBackingSize: NSOpenGLContextParameter =
        NSOpenGLContextParameterSurfaceBackingSize
);

extern_static!(
    NSOpenGLCPReclaimResources: NSOpenGLContextParameter = NSOpenGLContextParameterReclaimResources
);

extern_static!(
    NSOpenGLCPCurrentRendererID: NSOpenGLContextParameter =
        NSOpenGLContextParameterCurrentRendererID
);

extern_static!(
    NSOpenGLCPGPUVertexProcessing: NSOpenGLContextParameter =
        NSOpenGLContextParameterGPUVertexProcessing
);

extern_static!(
    NSOpenGLCPGPUFragmentProcessing: NSOpenGLContextParameter =
        NSOpenGLContextParameterGPUFragmentProcessing
);

extern_static!(
    NSOpenGLCPHasDrawable: NSOpenGLContextParameter = NSOpenGLContextParameterHasDrawable
);

extern_static!(
    NSOpenGLCPMPSwapsInFlight: NSOpenGLContextParameter = NSOpenGLContextParameterMPSwapsInFlight
);

extern_static!(
    NSOpenGLCPSwapRectangle: NSOpenGLContextParameter = NSOpenGLContextParameterSwapRectangle
);

extern_static!(
    NSOpenGLCPSwapRectangleEnable: NSOpenGLContextParameter =
        NSOpenGLContextParameterSwapRectangleEnable
);

extern_static!(
    NSOpenGLCPRasterizationEnable: NSOpenGLContextParameter =
        NSOpenGLContextParameterRasterizationEnable
);

extern_static!(
    NSOpenGLCPStateValidation: NSOpenGLContextParameter = NSOpenGLContextParameterStateValidation
);

extern_static!(
    NSOpenGLCPSurfaceSurfaceVolatile: NSOpenGLContextParameter =
        NSOpenGLContextParameterSurfaceSurfaceVolatile
);
