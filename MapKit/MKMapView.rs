//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[ns_enum]
#[underlying(NSInteger)]
pub enum MKUserTrackingMode {
    MKUserTrackingModeNone = 0,
    MKUserTrackingModeFollow = 1,
    MKUserTrackingModeFollowWithHeading = 2,
}

extern_static!(MKMapViewDefaultAnnotationViewReuseIdentifier: &'static NSString);

extern_static!(MKMapViewDefaultClusterAnnotationViewReuseIdentifier: &'static NSString);

#[objc2::interface(
    unsafe super = NSView,
    unsafe inherits = [
        NSResponder,
        NSObject,
    ]
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKMapView")]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub type MKMapView;
}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSAccessibility for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSAccessibilityElementProtocol for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSAnimatablePropertyContainer for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSAppearanceCustomization for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSCoding for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSDraggingDestination for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSObjectProtocol for MKMapView {}

#[cfg(feature = "MapKit_MKMapView")]
unsafe impl NSUserInterfaceItemIdentification for MKMapView {}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKMapView")]
    pub type MKMapView;

    #[objc2::method(sel = "delegate", managed = "Other")]
    pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn MKMapViewDelegate>>>;

    #[objc2::method(sel = "setDelegate:")]
    pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn MKMapViewDelegate>>);

    #[deprecated]
    #[objc2::method(sel = "mapType")]
    pub unsafe fn mapType(&self) -> MKMapType;

    #[deprecated]
    #[objc2::method(sel = "setMapType:")]
    pub unsafe fn setMapType(&self, map_type: MKMapType);

    #[cfg(feature = "MapKit_MKMapConfiguration")]
    #[objc2::method(sel = "preferredConfiguration", managed = "Other")]
    pub unsafe fn preferredConfiguration(&self) -> Id<MKMapConfiguration>;

    #[cfg(feature = "MapKit_MKMapConfiguration")]
    #[objc2::method(sel = "setPreferredConfiguration:")]
    pub unsafe fn setPreferredConfiguration(&self, preferred_configuration: &MKMapConfiguration);

    #[objc2::method(sel = "region")]
    pub unsafe fn region(&self) -> MKCoordinateRegion;

    #[objc2::method(sel = "setRegion:")]
    pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

    #[objc2::method(sel = "setRegion:animated:")]
    pub unsafe fn setRegion_animated(&self, region: MKCoordinateRegion, animated: bool);

    #[objc2::method(sel = "centerCoordinate")]
    pub unsafe fn centerCoordinate(&self) -> CLLocationCoordinate2D;

    #[objc2::method(sel = "setCenterCoordinate:")]
    pub unsafe fn setCenterCoordinate(&self, center_coordinate: CLLocationCoordinate2D);

    #[objc2::method(sel = "setCenterCoordinate:animated:")]
    pub unsafe fn setCenterCoordinate_animated(
        &self,
        coordinate: CLLocationCoordinate2D,
        animated: bool,
    );

    #[objc2::method(sel = "regionThatFits:")]
    pub unsafe fn regionThatFits(&self, region: MKCoordinateRegion) -> MKCoordinateRegion;

    #[objc2::method(sel = "visibleMapRect")]
    pub unsafe fn visibleMapRect(&self) -> MKMapRect;

    #[objc2::method(sel = "setVisibleMapRect:")]
    pub unsafe fn setVisibleMapRect(&self, visible_map_rect: MKMapRect);

    #[objc2::method(sel = "setVisibleMapRect:animated:")]
    pub unsafe fn setVisibleMapRect_animated(&self, map_rect: MKMapRect, animate: bool);

    #[objc2::method(sel = "mapRectThatFits:")]
    pub unsafe fn mapRectThatFits(&self, map_rect: MKMapRect) -> MKMapRect;

    #[objc2::method(sel = "setVisibleMapRect:edgePadding:animated:")]
    pub unsafe fn setVisibleMapRect_edgePadding_animated(
        &self,
        map_rect: MKMapRect,
        insets: NSEdgeInsets,
        animate: bool,
    );

    #[objc2::method(sel = "mapRectThatFits:edgePadding:")]
    pub unsafe fn mapRectThatFits_edgePadding(
        &self,
        map_rect: MKMapRect,
        insets: NSEdgeInsets,
    ) -> MKMapRect;

    #[cfg(feature = "MapKit_MKMapCamera")]
    #[objc2::method(sel = "camera", managed = "Other")]
    pub unsafe fn camera(&self) -> Id<MKMapCamera>;

    #[cfg(feature = "MapKit_MKMapCamera")]
    #[objc2::method(sel = "setCamera:")]
    pub unsafe fn setCamera(&self, camera: &MKMapCamera);

    #[cfg(feature = "MapKit_MKMapCamera")]
    #[objc2::method(sel = "setCamera:animated:")]
    pub unsafe fn setCamera_animated(&self, camera: &MKMapCamera, animated: bool);

    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    #[objc2::method(sel = "cameraZoomRange", managed = "Other")]
    pub unsafe fn cameraZoomRange(&self) -> Id<MKMapCameraZoomRange>;

    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    #[objc2::method(sel = "setCameraZoomRange:")]
    pub unsafe fn setCameraZoomRange(&self, camera_zoom_range: Option<&MKMapCameraZoomRange>);

    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    #[objc2::method(sel = "setCameraZoomRange:animated:")]
    pub unsafe fn setCameraZoomRange_animated(
        &self,
        camera_zoom_range: Option<&MKMapCameraZoomRange>,
        animated: bool,
    );

    #[cfg(feature = "MapKit_MKMapCameraBoundary")]
    #[objc2::method(sel = "cameraBoundary", managed = "Other")]
    pub unsafe fn cameraBoundary(&self) -> Option<Id<MKMapCameraBoundary>>;

    #[cfg(feature = "MapKit_MKMapCameraBoundary")]
    #[objc2::method(sel = "setCameraBoundary:")]
    pub unsafe fn setCameraBoundary(&self, camera_boundary: Option<&MKMapCameraBoundary>);

    #[cfg(feature = "MapKit_MKMapCameraBoundary")]
    #[objc2::method(sel = "setCameraBoundary:animated:")]
    pub unsafe fn setCameraBoundary_animated(
        &self,
        camera_boundary: Option<&MKMapCameraBoundary>,
        animated: bool,
    );

    #[objc2::method(sel = "convertCoordinate:toPointToView:")]
    pub unsafe fn convertCoordinate_toPointToView(
        &self,
        coordinate: CLLocationCoordinate2D,
        view: Option<&NSView>,
    ) -> CGPoint;

    #[objc2::method(sel = "convertPoint:toCoordinateFromView:")]
    pub unsafe fn convertPoint_toCoordinateFromView(
        &self,
        point: CGPoint,
        view: Option<&NSView>,
    ) -> CLLocationCoordinate2D;

    #[objc2::method(sel = "convertRegion:toRectToView:")]
    pub unsafe fn convertRegion_toRectToView(
        &self,
        region: MKCoordinateRegion,
        view: Option<&NSView>,
    ) -> CGRect;

    #[objc2::method(sel = "convertRect:toRegionFromView:")]
    pub unsafe fn convertRect_toRegionFromView(
        &self,
        rect: CGRect,
        view: Option<&NSView>,
    ) -> MKCoordinateRegion;

    #[objc2::method(sel = "isZoomEnabled")]
    pub unsafe fn isZoomEnabled(&self) -> bool;

    #[objc2::method(sel = "setZoomEnabled:")]
    pub unsafe fn setZoomEnabled(&self, zoom_enabled: bool);

    #[objc2::method(sel = "isScrollEnabled")]
    pub unsafe fn isScrollEnabled(&self) -> bool;

    #[objc2::method(sel = "setScrollEnabled:")]
    pub unsafe fn setScrollEnabled(&self, scroll_enabled: bool);

    #[objc2::method(sel = "isRotateEnabled")]
    pub unsafe fn isRotateEnabled(&self) -> bool;

    #[objc2::method(sel = "setRotateEnabled:")]
    pub unsafe fn setRotateEnabled(&self, rotate_enabled: bool);

    #[objc2::method(sel = "isPitchEnabled")]
    pub unsafe fn isPitchEnabled(&self) -> bool;

    #[objc2::method(sel = "setPitchEnabled:")]
    pub unsafe fn setPitchEnabled(&self, pitch_enabled: bool);

    #[objc2::method(sel = "showsPitchControl")]
    pub unsafe fn showsPitchControl(&self) -> bool;

    #[objc2::method(sel = "setShowsPitchControl:")]
    pub unsafe fn setShowsPitchControl(&self, shows_pitch_control: bool);

    #[objc2::method(sel = "showsZoomControls")]
    pub unsafe fn showsZoomControls(&self) -> bool;

    #[objc2::method(sel = "setShowsZoomControls:")]
    pub unsafe fn setShowsZoomControls(&self, shows_zoom_controls: bool);

    #[objc2::method(sel = "showsCompass")]
    pub unsafe fn showsCompass(&self) -> bool;

    #[objc2::method(sel = "setShowsCompass:")]
    pub unsafe fn setShowsCompass(&self, shows_compass: bool);

    #[objc2::method(sel = "showsScale")]
    pub unsafe fn showsScale(&self) -> bool;

    #[objc2::method(sel = "setShowsScale:")]
    pub unsafe fn setShowsScale(&self, shows_scale: bool);

    #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
    #[deprecated]
    #[objc2::method(sel = "pointOfInterestFilter", managed = "Other")]
    pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

    #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
    #[deprecated]
    #[objc2::method(sel = "setPointOfInterestFilter:")]
    pub unsafe fn setPointOfInterestFilter(
        &self,
        point_of_interest_filter: Option<&MKPointOfInterestFilter>,
    );

    #[deprecated = "Use pointOfInterestFilter"]
    #[objc2::method(sel = "showsPointsOfInterest")]
    pub unsafe fn showsPointsOfInterest(&self) -> bool;

    #[deprecated = "Use pointOfInterestFilter"]
    #[objc2::method(sel = "setShowsPointsOfInterest:")]
    pub unsafe fn setShowsPointsOfInterest(&self, shows_points_of_interest: bool);

    #[deprecated = "None"]
    #[objc2::method(sel = "showsBuildings")]
    pub unsafe fn showsBuildings(&self) -> bool;

    #[deprecated = "None"]
    #[objc2::method(sel = "setShowsBuildings:")]
    pub unsafe fn setShowsBuildings(&self, shows_buildings: bool);

    #[deprecated]
    #[objc2::method(sel = "showsTraffic")]
    pub unsafe fn showsTraffic(&self) -> bool;

    #[deprecated]
    #[objc2::method(sel = "setShowsTraffic:")]
    pub unsafe fn setShowsTraffic(&self, shows_traffic: bool);

    #[objc2::method(sel = "showsUserLocation")]
    pub unsafe fn showsUserLocation(&self) -> bool;

    #[objc2::method(sel = "setShowsUserLocation:")]
    pub unsafe fn setShowsUserLocation(&self, shows_user_location: bool);

    #[cfg(feature = "MapKit_MKUserLocation")]
    #[objc2::method(sel = "userLocation", managed = "Other")]
    pub unsafe fn userLocation(&self) -> Id<MKUserLocation>;

    #[objc2::method(sel = "userTrackingMode")]
    pub unsafe fn userTrackingMode(&self) -> MKUserTrackingMode;

    #[objc2::method(sel = "setUserTrackingMode:")]
    pub unsafe fn setUserTrackingMode(&self, user_tracking_mode: MKUserTrackingMode);

    #[objc2::method(sel = "setUserTrackingMode:animated:")]
    pub unsafe fn setUserTrackingMode_animated(&self, mode: MKUserTrackingMode, animated: bool);

    #[objc2::method(sel = "isUserLocationVisible")]
    pub unsafe fn isUserLocationVisible(&self) -> bool;

    #[objc2::method(sel = "addAnnotation:")]
    pub unsafe fn addAnnotation(&self, annotation: &ProtocolObject<dyn MKAnnotation>);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "addAnnotations:")]
    pub unsafe fn addAnnotations(&self, annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>);

    #[objc2::method(sel = "removeAnnotation:")]
    pub unsafe fn removeAnnotation(&self, annotation: &ProtocolObject<dyn MKAnnotation>);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "removeAnnotations:")]
    pub unsafe fn removeAnnotations(&self, annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "annotations", managed = "Other")]
    pub unsafe fn annotations(&self) -> Id<NSArray<ProtocolObject<dyn MKAnnotation>>>;

    #[cfg(feature = "Foundation_NSSet")]
    #[objc2::method(sel = "annotationsInMapRect:", managed = "Other")]
    pub unsafe fn annotationsInMapRect(
        &self,
        map_rect: MKMapRect,
    ) -> Id<NSSet<ProtocolObject<dyn MKAnnotation>>>;

    #[cfg(feature = "MapKit_MKAnnotationView")]
    #[objc2::method(sel = "viewForAnnotation:", managed = "Other")]
    pub unsafe fn viewForAnnotation(
        &self,
        annotation: &ProtocolObject<dyn MKAnnotation>,
    ) -> Option<Id<MKAnnotationView>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "MapKit_MKAnnotationView"))]
    #[objc2::method(
        sel = "dequeueReusableAnnotationViewWithIdentifier:",
        managed = "Other"
    )]
    pub unsafe fn dequeueReusableAnnotationViewWithIdentifier(
        &self,
        identifier: &NSString,
    ) -> Option<Id<MKAnnotationView>>;

    #[cfg(all(feature = "Foundation_NSString", feature = "MapKit_MKAnnotationView"))]
    #[objc2::method(
        sel = "dequeueReusableAnnotationViewWithIdentifier:forAnnotation:",
        managed = "Other"
    )]
    pub unsafe fn dequeueReusableAnnotationViewWithIdentifier_forAnnotation(
        &self,
        identifier: &NSString,
        annotation: &ProtocolObject<dyn MKAnnotation>,
    ) -> Id<MKAnnotationView>;

    #[cfg(feature = "Foundation_NSString")]
    #[objc2::method(sel = "registerClass:forAnnotationViewWithReuseIdentifier:")]
    pub unsafe fn registerClass_forAnnotationViewWithReuseIdentifier(
        &self,
        view_class: Option<&Class>,
        identifier: &NSString,
    );

    #[objc2::method(sel = "selectAnnotation:animated:")]
    pub unsafe fn selectAnnotation_animated(
        &self,
        annotation: &ProtocolObject<dyn MKAnnotation>,
        animated: bool,
    );

    #[objc2::method(sel = "deselectAnnotation:animated:")]
    pub unsafe fn deselectAnnotation_animated(
        &self,
        annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
        animated: bool,
    );

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "selectedAnnotations", managed = "Other")]
    pub unsafe fn selectedAnnotations(&self) -> Id<NSArray<ProtocolObject<dyn MKAnnotation>>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "setSelectedAnnotations:")]
    pub unsafe fn setSelectedAnnotations(
        &self,
        selected_annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
    );

    #[objc2::method(sel = "annotationVisibleRect")]
    pub unsafe fn annotationVisibleRect(&self) -> CGRect;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "showAnnotations:animated:")]
    pub unsafe fn showAnnotations_animated(
        &self,
        annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
        animated: bool,
    );
}

#[ns_enum]
#[underlying(NSInteger)]
pub enum MKOverlayLevel {
    MKOverlayLevelAboveRoads = 0,
    MKOverlayLevelAboveLabels = 1,
}

#[objc2::interface(
    unsafe continue,
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKMapView")]
    pub type MKMapView;

    #[objc2::method(sel = "addOverlay:level:")]
    pub unsafe fn addOverlay_level(
        &self,
        overlay: &ProtocolObject<dyn MKOverlay>,
        level: MKOverlayLevel,
    );

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "addOverlays:level:")]
    pub unsafe fn addOverlays_level(
        &self,
        overlays: &NSArray<ProtocolObject<dyn MKOverlay>>,
        level: MKOverlayLevel,
    );

    #[objc2::method(sel = "removeOverlay:")]
    pub unsafe fn removeOverlay(&self, overlay: &ProtocolObject<dyn MKOverlay>);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "removeOverlays:")]
    pub unsafe fn removeOverlays(&self, overlays: &NSArray<ProtocolObject<dyn MKOverlay>>);

    #[objc2::method(sel = "insertOverlay:atIndex:level:")]
    pub unsafe fn insertOverlay_atIndex_level(
        &self,
        overlay: &ProtocolObject<dyn MKOverlay>,
        index: NSUInteger,
        level: MKOverlayLevel,
    );

    #[objc2::method(sel = "insertOverlay:aboveOverlay:")]
    pub unsafe fn insertOverlay_aboveOverlay(
        &self,
        overlay: &ProtocolObject<dyn MKOverlay>,
        sibling: &ProtocolObject<dyn MKOverlay>,
    );

    #[objc2::method(sel = "insertOverlay:belowOverlay:")]
    pub unsafe fn insertOverlay_belowOverlay(
        &self,
        overlay: &ProtocolObject<dyn MKOverlay>,
        sibling: &ProtocolObject<dyn MKOverlay>,
    );

    #[objc2::method(sel = "exchangeOverlay:withOverlay:")]
    pub unsafe fn exchangeOverlay_withOverlay(
        &self,
        overlay1: &ProtocolObject<dyn MKOverlay>,
        overlay2: &ProtocolObject<dyn MKOverlay>,
    );

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "overlays", managed = "Other")]
    pub unsafe fn overlays(&self) -> Id<NSArray<ProtocolObject<dyn MKOverlay>>>;

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "overlaysInLevel:", managed = "Other")]
    pub unsafe fn overlaysInLevel(
        &self,
        level: MKOverlayLevel,
    ) -> Id<NSArray<ProtocolObject<dyn MKOverlay>>>;

    #[cfg(feature = "MapKit_MKOverlayRenderer")]
    #[objc2::method(sel = "rendererForOverlay:", managed = "Other")]
    pub unsafe fn rendererForOverlay(
        &self,
        overlay: &ProtocolObject<dyn MKOverlay>,
    ) -> Option<Id<MKOverlayRenderer>>;

    #[objc2::method(sel = "addOverlay:")]
    pub unsafe fn addOverlay(&self, overlay: &ProtocolObject<dyn MKOverlay>);

    #[cfg(feature = "Foundation_NSArray")]
    #[objc2::method(sel = "addOverlays:")]
    pub unsafe fn addOverlays(&self, overlays: &NSArray<ProtocolObject<dyn MKOverlay>>);

    #[objc2::method(sel = "insertOverlay:atIndex:")]
    pub unsafe fn insertOverlay_atIndex(
        &self,
        overlay: &ProtocolObject<dyn MKOverlay>,
        index: NSUInteger,
    );

    #[objc2::method(sel = "exchangeOverlayAtIndex:withOverlayAtIndex:")]
    pub unsafe fn exchangeOverlayAtIndex_withOverlayAtIndex(
        &self,
        index1: NSUInteger,
        index2: NSUInteger,
    );
}

#[objc2::protocol]
pub unsafe trait MKMapViewDelegate: NSObjectProtocol {
    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapView:regionWillChangeAnimated:")]
    unsafe fn mapView_regionWillChangeAnimated(&self, map_view: &MKMapView, animated: bool);

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapView:regionDidChangeAnimated:")]
    unsafe fn mapView_regionDidChangeAnimated(&self, map_view: &MKMapView, animated: bool);

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapViewDidChangeVisibleRegion:")]
    unsafe fn mapViewDidChangeVisibleRegion(&self, map_view: &MKMapView);

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapViewWillStartLoadingMap:")]
    unsafe fn mapViewWillStartLoadingMap(&self, map_view: &MKMapView);

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapViewDidFinishLoadingMap:")]
    unsafe fn mapViewDidFinishLoadingMap(&self, map_view: &MKMapView);

    #[cfg(all(feature = "Foundation_NSError", feature = "MapKit_MKMapView"))]
    #[objc2::method(optional, sel = "mapViewDidFailLoadingMap:withError:")]
    unsafe fn mapViewDidFailLoadingMap_withError(&self, map_view: &MKMapView, error: &NSError);

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapViewWillStartRenderingMap:")]
    unsafe fn mapViewWillStartRenderingMap(&self, map_view: &MKMapView);

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapViewDidFinishRenderingMap:fullyRendered:")]
    unsafe fn mapViewDidFinishRenderingMap_fullyRendered(
        &self,
        map_view: &MKMapView,
        fully_rendered: bool,
    );

    #[cfg(all(feature = "MapKit_MKAnnotationView", feature = "MapKit_MKMapView"))]
    #[objc2::method(optional, sel = "mapView:viewForAnnotation:", managed = "Other")]
    unsafe fn mapView_viewForAnnotation(
        &self,
        map_view: &MKMapView,
        annotation: &ProtocolObject<dyn MKAnnotation>,
    ) -> Option<Id<MKAnnotationView>>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "MapKit_MKAnnotationView",
        feature = "MapKit_MKMapView"
    ))]
    #[objc2::method(optional, sel = "mapView:didAddAnnotationViews:")]
    unsafe fn mapView_didAddAnnotationViews(
        &self,
        map_view: &MKMapView,
        views: &NSArray<MKAnnotationView>,
    );

    #[cfg(all(feature = "MapKit_MKAnnotationView", feature = "MapKit_MKMapView"))]
    #[objc2::method(optional, sel = "mapView:didSelectAnnotationView:")]
    unsafe fn mapView_didSelectAnnotationView(&self, map_view: &MKMapView, view: &MKAnnotationView);

    #[cfg(all(feature = "MapKit_MKAnnotationView", feature = "MapKit_MKMapView"))]
    #[objc2::method(optional, sel = "mapView:didDeselectAnnotationView:")]
    unsafe fn mapView_didDeselectAnnotationView(
        &self,
        map_view: &MKMapView,
        view: &MKAnnotationView,
    );

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapView:didSelectAnnotation:")]
    unsafe fn mapView_didSelectAnnotation(
        &self,
        map_view: &MKMapView,
        annotation: &ProtocolObject<dyn MKAnnotation>,
    );

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapView:didDeselectAnnotation:")]
    unsafe fn mapView_didDeselectAnnotation(
        &self,
        map_view: &MKMapView,
        annotation: &ProtocolObject<dyn MKAnnotation>,
    );

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapViewWillStartLocatingUser:")]
    unsafe fn mapViewWillStartLocatingUser(&self, map_view: &MKMapView);

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapViewDidStopLocatingUser:")]
    unsafe fn mapViewDidStopLocatingUser(&self, map_view: &MKMapView);

    #[cfg(all(feature = "MapKit_MKMapView", feature = "MapKit_MKUserLocation"))]
    #[objc2::method(optional, sel = "mapView:didUpdateUserLocation:")]
    unsafe fn mapView_didUpdateUserLocation(
        &self,
        map_view: &MKMapView,
        user_location: &MKUserLocation,
    );

    #[cfg(all(feature = "Foundation_NSError", feature = "MapKit_MKMapView"))]
    #[objc2::method(optional, sel = "mapView:didFailToLocateUserWithError:")]
    unsafe fn mapView_didFailToLocateUserWithError(&self, map_view: &MKMapView, error: &NSError);

    #[cfg(all(feature = "MapKit_MKAnnotationView", feature = "MapKit_MKMapView"))]
    #[objc2::method(
        optional,
        sel = "mapView:annotationView:didChangeDragState:fromOldState:"
    )]
    unsafe fn mapView_annotationView_didChangeDragState_fromOldState(
        &self,
        map_view: &MKMapView,
        view: &MKAnnotationView,
        new_state: MKAnnotationViewDragState,
        old_state: MKAnnotationViewDragState,
    );

    #[cfg(feature = "MapKit_MKMapView")]
    #[objc2::method(optional, sel = "mapView:didChangeUserTrackingMode:animated:")]
    unsafe fn mapView_didChangeUserTrackingMode_animated(
        &self,
        map_view: &MKMapView,
        mode: MKUserTrackingMode,
        animated: bool,
    );

    #[cfg(all(feature = "MapKit_MKMapView", feature = "MapKit_MKOverlayRenderer"))]
    #[objc2::method(optional, sel = "mapView:rendererForOverlay:", managed = "Other")]
    unsafe fn mapView_rendererForOverlay(
        &self,
        map_view: &MKMapView,
        overlay: &ProtocolObject<dyn MKOverlay>,
    ) -> Id<MKOverlayRenderer>;

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "MapKit_MKMapView",
        feature = "MapKit_MKOverlayRenderer"
    ))]
    #[objc2::method(optional, sel = "mapView:didAddOverlayRenderers:")]
    unsafe fn mapView_didAddOverlayRenderers(
        &self,
        map_view: &MKMapView,
        renderers: &NSArray<MKOverlayRenderer>,
    );

    #[cfg(all(
        feature = "Foundation_NSArray",
        feature = "MapKit_MKClusterAnnotation",
        feature = "MapKit_MKMapView"
    ))]
    #[objc2::method(
        optional,
        sel = "mapView:clusterAnnotationForMemberAnnotations:",
        managed = "Other"
    )]
    unsafe fn mapView_clusterAnnotationForMemberAnnotations(
        &self,
        map_view: &MKMapView,
        member_annotations: &NSArray<ProtocolObject<dyn MKAnnotation>>,
    ) -> Id<MKClusterAnnotation>;
}

#[objc2::interface(
    unsafe continue,
    impl_attrs = {
        /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKMapView")]
    }
)]
extern "Objective-C" {
    #[cfg(feature = "MapKit_MKMapView")]
    pub type MKMapView;

    #[objc2::method(sel = "initWithFrame:", managed = "Init")]
    pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
}
