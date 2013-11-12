use std::libc::*;
use _unsafe::*;
use base::*;
use core::*;
use _unavailable::*;

pub struct cbAntiflickerPlugin(*mut c_void);
impl _cbAntiflickerPlugin for cbAntiflickerPlugin {}
impl _cbPluginBase for cbAntiflickerPlugin {}
impl _wxEvtHandler for cbAntiflickerPlugin {}
impl _wxObject for cbAntiflickerPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbAntiflickerPlugin {
    pub fn from(handle: *mut c_void) -> @cbAntiflickerPlugin { @cbAntiflickerPlugin(handle) }
    pub fn null() -> @cbAntiflickerPlugin { cbAntiflickerPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbAntiflickerPlugin : _cbPluginBase {
}

pub struct cbBarDragPlugin(*mut c_void);
impl _cbBarDragPlugin for cbBarDragPlugin {}
impl _cbPluginBase for cbBarDragPlugin {}
impl _wxEvtHandler for cbBarDragPlugin {}
impl _wxObject for cbBarDragPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbBarDragPlugin {
    pub fn from(handle: *mut c_void) -> @cbBarDragPlugin { @cbBarDragPlugin(handle) }
    pub fn null() -> @cbBarDragPlugin { cbBarDragPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbBarDragPlugin : _cbPluginBase {
}

pub struct cbBarHintsPlugin(*mut c_void);
impl _cbBarHintsPlugin for cbBarHintsPlugin {}
impl _cbPluginBase for cbBarHintsPlugin {}
impl _wxEvtHandler for cbBarHintsPlugin {}
impl _wxObject for cbBarHintsPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbBarHintsPlugin {
    pub fn from(handle: *mut c_void) -> @cbBarHintsPlugin { @cbBarHintsPlugin(handle) }
    pub fn null() -> @cbBarHintsPlugin { cbBarHintsPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbBarHintsPlugin : _cbPluginBase {
}

pub struct cbBarInfo(*mut c_void);
impl _cbBarInfo for cbBarInfo {}
impl _wxObject for cbBarInfo { fn handle(&self) -> *mut c_void { **self } }

impl cbBarInfo {
    pub fn from(handle: *mut c_void) -> @cbBarInfo { @cbBarInfo(handle) }
    pub fn null() -> @cbBarInfo { cbBarInfo::from(0 as *mut c_void) }
    
}

pub trait _cbBarInfo : _wxObject {
}

pub struct cbBarSpy(*mut c_void);
impl _cbBarSpy for cbBarSpy {}
impl _wxEvtHandler for cbBarSpy {}
impl _wxObject for cbBarSpy { fn handle(&self) -> *mut c_void { **self } }

impl cbBarSpy {
    pub fn from(handle: *mut c_void) -> @cbBarSpy { @cbBarSpy(handle) }
    pub fn null() -> @cbBarSpy { cbBarSpy::from(0 as *mut c_void) }
    
}

pub trait _cbBarSpy : _wxEvtHandler {
}

pub struct cbCloseBox(*mut c_void);
impl _cbCloseBox for cbCloseBox {}
impl _cbMiniButton for cbCloseBox {}
impl _wxObject for cbCloseBox { fn handle(&self) -> *mut c_void { **self } }

impl cbCloseBox {
    pub fn from(handle: *mut c_void) -> @cbCloseBox { @cbCloseBox(handle) }
    pub fn null() -> @cbCloseBox { cbCloseBox::from(0 as *mut c_void) }
    
}

pub trait _cbCloseBox : _cbMiniButton {
}

pub struct cbCollapseBox(*mut c_void);
impl _cbCollapseBox for cbCollapseBox {}
impl _cbMiniButton for cbCollapseBox {}
impl _wxObject for cbCollapseBox { fn handle(&self) -> *mut c_void { **self } }

impl cbCollapseBox {
    pub fn from(handle: *mut c_void) -> @cbCollapseBox { @cbCollapseBox(handle) }
    pub fn null() -> @cbCollapseBox { cbCollapseBox::from(0 as *mut c_void) }
    
}

pub trait _cbCollapseBox : _cbMiniButton {
}

pub struct cbCommonPaneProperties(*mut c_void);
impl _cbCommonPaneProperties for cbCommonPaneProperties {}
impl _wxObject for cbCommonPaneProperties { fn handle(&self) -> *mut c_void { **self } }

impl cbCommonPaneProperties {
    pub fn from(handle: *mut c_void) -> @cbCommonPaneProperties { @cbCommonPaneProperties(handle) }
    pub fn null() -> @cbCommonPaneProperties { cbCommonPaneProperties::from(0 as *mut c_void) }
    
}

pub trait _cbCommonPaneProperties : _wxObject {
}

pub struct cbCustomizeBarEvent(*mut c_void);
impl _cbCustomizeBarEvent for cbCustomizeBarEvent {}
impl _cbPluginEvent for cbCustomizeBarEvent {}
impl _wxEvent for cbCustomizeBarEvent {}
impl _wxObject for cbCustomizeBarEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbCustomizeBarEvent {
    pub fn from(handle: *mut c_void) -> @cbCustomizeBarEvent { @cbCustomizeBarEvent(handle) }
    pub fn null() -> @cbCustomizeBarEvent { cbCustomizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait _cbCustomizeBarEvent : _cbPluginEvent {
}

pub struct cbCustomizeLayoutEvent(*mut c_void);
impl _cbCustomizeLayoutEvent for cbCustomizeLayoutEvent {}
impl _cbPluginEvent for cbCustomizeLayoutEvent {}
impl _wxEvent for cbCustomizeLayoutEvent {}
impl _wxObject for cbCustomizeLayoutEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbCustomizeLayoutEvent {
    pub fn from(handle: *mut c_void) -> @cbCustomizeLayoutEvent { @cbCustomizeLayoutEvent(handle) }
    pub fn null() -> @cbCustomizeLayoutEvent { cbCustomizeLayoutEvent::from(0 as *mut c_void) }
    
}

pub trait _cbCustomizeLayoutEvent : _cbPluginEvent {
}

pub struct cbDimHandlerBase(*mut c_void);
impl _cbDimHandlerBase for cbDimHandlerBase {}
impl _wxObject for cbDimHandlerBase { fn handle(&self) -> *mut c_void { **self } }

impl cbDimHandlerBase {
    pub fn from(handle: *mut c_void) -> @cbDimHandlerBase { @cbDimHandlerBase(handle) }
    pub fn null() -> @cbDimHandlerBase { cbDimHandlerBase::from(0 as *mut c_void) }
    
}

pub trait _cbDimHandlerBase : _wxObject {
}

pub struct cbDimInfo(*mut c_void);
impl _cbDimInfo for cbDimInfo {}
impl _wxObject for cbDimInfo { fn handle(&self) -> *mut c_void { **self } }

impl cbDimInfo {
    pub fn from(handle: *mut c_void) -> @cbDimInfo { @cbDimInfo(handle) }
    pub fn null() -> @cbDimInfo { cbDimInfo::from(0 as *mut c_void) }
    
}

pub trait _cbDimInfo : _wxObject {
}

pub struct cbDockBox(*mut c_void);
impl _cbDockBox for cbDockBox {}
impl _cbMiniButton for cbDockBox {}
impl _wxObject for cbDockBox { fn handle(&self) -> *mut c_void { **self } }

impl cbDockBox {
    pub fn from(handle: *mut c_void) -> @cbDockBox { @cbDockBox(handle) }
    pub fn null() -> @cbDockBox { cbDockBox::from(0 as *mut c_void) }
    
}

pub trait _cbDockBox : _cbMiniButton {
}

pub struct cbDockPane(*mut c_void);
impl _cbDockPane for cbDockPane {}
impl _wxObject for cbDockPane { fn handle(&self) -> *mut c_void { **self } }

impl cbDockPane {
    pub fn from(handle: *mut c_void) -> @cbDockPane { @cbDockPane(handle) }
    pub fn null() -> @cbDockPane { cbDockPane::from(0 as *mut c_void) }
    
}

pub trait _cbDockPane : _wxObject {
}

pub struct cbDrawBarDecorEvent(*mut c_void);
impl _cbDrawBarDecorEvent for cbDrawBarDecorEvent {}
impl _cbPluginEvent for cbDrawBarDecorEvent {}
impl _wxEvent for cbDrawBarDecorEvent {}
impl _wxObject for cbDrawBarDecorEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawBarDecorEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawBarDecorEvent { @cbDrawBarDecorEvent(handle) }
    pub fn null() -> @cbDrawBarDecorEvent { cbDrawBarDecorEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawBarDecorEvent : _cbPluginEvent {
}

pub struct cbDrawBarHandlesEvent(*mut c_void);
impl _cbDrawBarHandlesEvent for cbDrawBarHandlesEvent {}
impl _cbPluginEvent for cbDrawBarHandlesEvent {}
impl _wxEvent for cbDrawBarHandlesEvent {}
impl _wxObject for cbDrawBarHandlesEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawBarHandlesEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawBarHandlesEvent { @cbDrawBarHandlesEvent(handle) }
    pub fn null() -> @cbDrawBarHandlesEvent { cbDrawBarHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawBarHandlesEvent : _cbPluginEvent {
}

pub struct cbDrawHintRectEvent(*mut c_void);
impl _cbDrawHintRectEvent for cbDrawHintRectEvent {}
impl _cbPluginEvent for cbDrawHintRectEvent {}
impl _wxEvent for cbDrawHintRectEvent {}
impl _wxObject for cbDrawHintRectEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawHintRectEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawHintRectEvent { @cbDrawHintRectEvent(handle) }
    pub fn null() -> @cbDrawHintRectEvent { cbDrawHintRectEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawHintRectEvent : _cbPluginEvent {
}

pub struct cbDrawPaneBkGroundEvent(*mut c_void);
impl _cbDrawPaneBkGroundEvent for cbDrawPaneBkGroundEvent {}
impl _cbPluginEvent for cbDrawPaneBkGroundEvent {}
impl _wxEvent for cbDrawPaneBkGroundEvent {}
impl _wxObject for cbDrawPaneBkGroundEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawPaneBkGroundEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawPaneBkGroundEvent { @cbDrawPaneBkGroundEvent(handle) }
    pub fn null() -> @cbDrawPaneBkGroundEvent { cbDrawPaneBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawPaneBkGroundEvent : _cbPluginEvent {
}

pub struct cbDrawPaneDecorEvent(*mut c_void);
impl _cbDrawPaneDecorEvent for cbDrawPaneDecorEvent {}
impl _cbPluginEvent for cbDrawPaneDecorEvent {}
impl _wxEvent for cbDrawPaneDecorEvent {}
impl _wxObject for cbDrawPaneDecorEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawPaneDecorEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawPaneDecorEvent { @cbDrawPaneDecorEvent(handle) }
    pub fn null() -> @cbDrawPaneDecorEvent { cbDrawPaneDecorEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawPaneDecorEvent : _cbPluginEvent {
}

pub struct cbDrawRowBkGroundEvent(*mut c_void);
impl _cbDrawRowBkGroundEvent for cbDrawRowBkGroundEvent {}
impl _cbPluginEvent for cbDrawRowBkGroundEvent {}
impl _wxEvent for cbDrawRowBkGroundEvent {}
impl _wxObject for cbDrawRowBkGroundEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawRowBkGroundEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawRowBkGroundEvent { @cbDrawRowBkGroundEvent(handle) }
    pub fn null() -> @cbDrawRowBkGroundEvent { cbDrawRowBkGroundEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawRowBkGroundEvent : _cbPluginEvent {
}

pub struct cbDrawRowDecorEvent(*mut c_void);
impl _cbDrawRowDecorEvent for cbDrawRowDecorEvent {}
impl _cbPluginEvent for cbDrawRowDecorEvent {}
impl _wxEvent for cbDrawRowDecorEvent {}
impl _wxObject for cbDrawRowDecorEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawRowDecorEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawRowDecorEvent { @cbDrawRowDecorEvent(handle) }
    pub fn null() -> @cbDrawRowDecorEvent { cbDrawRowDecorEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawRowDecorEvent : _cbPluginEvent {
}

pub struct cbDrawRowHandlesEvent(*mut c_void);
impl _cbDrawRowHandlesEvent for cbDrawRowHandlesEvent {}
impl _cbPluginEvent for cbDrawRowHandlesEvent {}
impl _wxEvent for cbDrawRowHandlesEvent {}
impl _wxObject for cbDrawRowHandlesEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbDrawRowHandlesEvent {
    pub fn from(handle: *mut c_void) -> @cbDrawRowHandlesEvent { @cbDrawRowHandlesEvent(handle) }
    pub fn null() -> @cbDrawRowHandlesEvent { cbDrawRowHandlesEvent::from(0 as *mut c_void) }
    
}

pub trait _cbDrawRowHandlesEvent : _cbPluginEvent {
}

pub struct cbDynToolBarDimHandler(*mut c_void);
impl _cbDynToolBarDimHandler for cbDynToolBarDimHandler {}
impl _cbDimHandlerBase for cbDynToolBarDimHandler {}
impl _wxObject for cbDynToolBarDimHandler { fn handle(&self) -> *mut c_void { **self } }

impl cbDynToolBarDimHandler {
    pub fn from(handle: *mut c_void) -> @cbDynToolBarDimHandler { @cbDynToolBarDimHandler(handle) }
    pub fn null() -> @cbDynToolBarDimHandler { cbDynToolBarDimHandler::from(0 as *mut c_void) }
    
}

pub trait _cbDynToolBarDimHandler : _cbDimHandlerBase {
}

pub struct cbFinishDrawInAreaEvent(*mut c_void);
impl _cbFinishDrawInAreaEvent for cbFinishDrawInAreaEvent {}
impl _cbPluginEvent for cbFinishDrawInAreaEvent {}
impl _wxEvent for cbFinishDrawInAreaEvent {}
impl _wxObject for cbFinishDrawInAreaEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbFinishDrawInAreaEvent {
    pub fn from(handle: *mut c_void) -> @cbFinishDrawInAreaEvent { @cbFinishDrawInAreaEvent(handle) }
    pub fn null() -> @cbFinishDrawInAreaEvent { cbFinishDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait _cbFinishDrawInAreaEvent : _cbPluginEvent {
}

pub struct cbFloatedBarWindow(*mut c_void);
impl _cbFloatedBarWindow for cbFloatedBarWindow {}
impl _wxToolWindow for cbFloatedBarWindow {}
impl _wxFrame for cbFloatedBarWindow {}
impl _wxTopLevelWindow for cbFloatedBarWindow {}
impl _wxWindow for cbFloatedBarWindow {}
impl _wxEvtHandler for cbFloatedBarWindow {}
impl _wxObject for cbFloatedBarWindow { fn handle(&self) -> *mut c_void { **self } }

impl cbFloatedBarWindow {
    pub fn from(handle: *mut c_void) -> @cbFloatedBarWindow { @cbFloatedBarWindow(handle) }
    pub fn null() -> @cbFloatedBarWindow { cbFloatedBarWindow::from(0 as *mut c_void) }
    
}

pub trait _cbFloatedBarWindow : _wxToolWindow {
}

pub struct cbGCUpdatesMgr(*mut c_void);
impl _cbGCUpdatesMgr for cbGCUpdatesMgr {}
impl _cbSimpleUpdatesMgr for cbGCUpdatesMgr {}
impl _cbUpdatesManagerBase for cbGCUpdatesMgr {}
impl _wxObject for cbGCUpdatesMgr { fn handle(&self) -> *mut c_void { **self } }

impl cbGCUpdatesMgr {
    pub fn from(handle: *mut c_void) -> @cbGCUpdatesMgr { @cbGCUpdatesMgr(handle) }
    pub fn null() -> @cbGCUpdatesMgr { cbGCUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait _cbGCUpdatesMgr : _cbSimpleUpdatesMgr {
}

pub struct cbHintAnimationPlugin(*mut c_void);
impl _cbHintAnimationPlugin for cbHintAnimationPlugin {}
impl _cbPluginBase for cbHintAnimationPlugin {}
impl _wxEvtHandler for cbHintAnimationPlugin {}
impl _wxObject for cbHintAnimationPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbHintAnimationPlugin {
    pub fn from(handle: *mut c_void) -> @cbHintAnimationPlugin { @cbHintAnimationPlugin(handle) }
    pub fn null() -> @cbHintAnimationPlugin { cbHintAnimationPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbHintAnimationPlugin : _cbPluginBase {
}

pub struct cbInsertBarEvent(*mut c_void);
impl _cbInsertBarEvent for cbInsertBarEvent {}
impl _cbPluginEvent for cbInsertBarEvent {}
impl _wxEvent for cbInsertBarEvent {}
impl _wxObject for cbInsertBarEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbInsertBarEvent {
    pub fn from(handle: *mut c_void) -> @cbInsertBarEvent { @cbInsertBarEvent(handle) }
    pub fn null() -> @cbInsertBarEvent { cbInsertBarEvent::from(0 as *mut c_void) }
    
}

pub trait _cbInsertBarEvent : _cbPluginEvent {
}

pub struct cbLayoutRowEvent(*mut c_void);
impl _cbLayoutRowEvent for cbLayoutRowEvent {}
impl _cbPluginEvent for cbLayoutRowEvent {}
impl _wxEvent for cbLayoutRowEvent {}
impl _wxObject for cbLayoutRowEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbLayoutRowEvent {
    pub fn from(handle: *mut c_void) -> @cbLayoutRowEvent { @cbLayoutRowEvent(handle) }
    pub fn null() -> @cbLayoutRowEvent { cbLayoutRowEvent::from(0 as *mut c_void) }
    
}

pub trait _cbLayoutRowEvent : _cbPluginEvent {
}

pub struct cbLeftDClickEvent(*mut c_void);
impl _cbLeftDClickEvent for cbLeftDClickEvent {}
impl _cbPluginEvent for cbLeftDClickEvent {}
impl _wxEvent for cbLeftDClickEvent {}
impl _wxObject for cbLeftDClickEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbLeftDClickEvent {
    pub fn from(handle: *mut c_void) -> @cbLeftDClickEvent { @cbLeftDClickEvent(handle) }
    pub fn null() -> @cbLeftDClickEvent { cbLeftDClickEvent::from(0 as *mut c_void) }
    
}

pub trait _cbLeftDClickEvent : _cbPluginEvent {
}

pub struct cbLeftDownEvent(*mut c_void);
impl _cbLeftDownEvent for cbLeftDownEvent {}
impl _cbPluginEvent for cbLeftDownEvent {}
impl _wxEvent for cbLeftDownEvent {}
impl _wxObject for cbLeftDownEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbLeftDownEvent {
    pub fn from(handle: *mut c_void) -> @cbLeftDownEvent { @cbLeftDownEvent(handle) }
    pub fn null() -> @cbLeftDownEvent { cbLeftDownEvent::from(0 as *mut c_void) }
    
}

pub trait _cbLeftDownEvent : _cbPluginEvent {
}

pub struct cbLeftUpEvent(*mut c_void);
impl _cbLeftUpEvent for cbLeftUpEvent {}
impl _cbPluginEvent for cbLeftUpEvent {}
impl _wxEvent for cbLeftUpEvent {}
impl _wxObject for cbLeftUpEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbLeftUpEvent {
    pub fn from(handle: *mut c_void) -> @cbLeftUpEvent { @cbLeftUpEvent(handle) }
    pub fn null() -> @cbLeftUpEvent { cbLeftUpEvent::from(0 as *mut c_void) }
    
}

pub trait _cbLeftUpEvent : _cbPluginEvent {
}

pub struct cbMiniButton(*mut c_void);
impl _cbMiniButton for cbMiniButton {}
impl _wxObject for cbMiniButton { fn handle(&self) -> *mut c_void { **self } }

impl cbMiniButton {
    pub fn from(handle: *mut c_void) -> @cbMiniButton { @cbMiniButton(handle) }
    pub fn null() -> @cbMiniButton { cbMiniButton::from(0 as *mut c_void) }
    
}

pub trait _cbMiniButton : _wxObject {
}

pub struct cbMotionEvent(*mut c_void);
impl _cbMotionEvent for cbMotionEvent {}
impl _cbPluginEvent for cbMotionEvent {}
impl _wxEvent for cbMotionEvent {}
impl _wxObject for cbMotionEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbMotionEvent {
    pub fn from(handle: *mut c_void) -> @cbMotionEvent { @cbMotionEvent(handle) }
    pub fn null() -> @cbMotionEvent { cbMotionEvent::from(0 as *mut c_void) }
    
}

pub trait _cbMotionEvent : _cbPluginEvent {
}

pub struct cbPaneDrawPlugin(*mut c_void);
impl _cbPaneDrawPlugin for cbPaneDrawPlugin {}
impl _cbPluginBase for cbPaneDrawPlugin {}
impl _wxEvtHandler for cbPaneDrawPlugin {}
impl _wxObject for cbPaneDrawPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbPaneDrawPlugin {
    pub fn from(handle: *mut c_void) -> @cbPaneDrawPlugin { @cbPaneDrawPlugin(handle) }
    pub fn null() -> @cbPaneDrawPlugin { cbPaneDrawPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbPaneDrawPlugin : _cbPluginBase {
}

pub struct cbPluginBase(*mut c_void);
impl _cbPluginBase for cbPluginBase {}
impl _wxEvtHandler for cbPluginBase {}
impl _wxObject for cbPluginBase { fn handle(&self) -> *mut c_void { **self } }

impl cbPluginBase {
    pub fn from(handle: *mut c_void) -> @cbPluginBase { @cbPluginBase(handle) }
    pub fn null() -> @cbPluginBase { cbPluginBase::from(0 as *mut c_void) }
    
}

pub trait _cbPluginBase : _wxEvtHandler {
}

pub struct cbPluginEvent(*mut c_void);
impl _cbPluginEvent for cbPluginEvent {}
impl _wxEvent for cbPluginEvent {}
impl _wxObject for cbPluginEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbPluginEvent {
    pub fn from(handle: *mut c_void) -> @cbPluginEvent { @cbPluginEvent(handle) }
    pub fn null() -> @cbPluginEvent { cbPluginEvent::from(0 as *mut c_void) }
    
}

pub trait _cbPluginEvent : _wxEvent {
}

pub struct cbRemoveBarEvent(*mut c_void);
impl _cbRemoveBarEvent for cbRemoveBarEvent {}
impl _cbPluginEvent for cbRemoveBarEvent {}
impl _wxEvent for cbRemoveBarEvent {}
impl _wxObject for cbRemoveBarEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbRemoveBarEvent {
    pub fn from(handle: *mut c_void) -> @cbRemoveBarEvent { @cbRemoveBarEvent(handle) }
    pub fn null() -> @cbRemoveBarEvent { cbRemoveBarEvent::from(0 as *mut c_void) }
    
}

pub trait _cbRemoveBarEvent : _cbPluginEvent {
}

pub struct cbResizeBarEvent(*mut c_void);
impl _cbResizeBarEvent for cbResizeBarEvent {}
impl _cbPluginEvent for cbResizeBarEvent {}
impl _wxEvent for cbResizeBarEvent {}
impl _wxObject for cbResizeBarEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbResizeBarEvent {
    pub fn from(handle: *mut c_void) -> @cbResizeBarEvent { @cbResizeBarEvent(handle) }
    pub fn null() -> @cbResizeBarEvent { cbResizeBarEvent::from(0 as *mut c_void) }
    
}

pub trait _cbResizeBarEvent : _cbPluginEvent {
}

pub struct cbResizeRowEvent(*mut c_void);
impl _cbResizeRowEvent for cbResizeRowEvent {}
impl _cbPluginEvent for cbResizeRowEvent {}
impl _wxEvent for cbResizeRowEvent {}
impl _wxObject for cbResizeRowEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbResizeRowEvent {
    pub fn from(handle: *mut c_void) -> @cbResizeRowEvent { @cbResizeRowEvent(handle) }
    pub fn null() -> @cbResizeRowEvent { cbResizeRowEvent::from(0 as *mut c_void) }
    
}

pub trait _cbResizeRowEvent : _cbPluginEvent {
}

pub struct cbRightDownEvent(*mut c_void);
impl _cbRightDownEvent for cbRightDownEvent {}
impl _cbPluginEvent for cbRightDownEvent {}
impl _wxEvent for cbRightDownEvent {}
impl _wxObject for cbRightDownEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbRightDownEvent {
    pub fn from(handle: *mut c_void) -> @cbRightDownEvent { @cbRightDownEvent(handle) }
    pub fn null() -> @cbRightDownEvent { cbRightDownEvent::from(0 as *mut c_void) }
    
}

pub trait _cbRightDownEvent : _cbPluginEvent {
}

pub struct cbRightUpEvent(*mut c_void);
impl _cbRightUpEvent for cbRightUpEvent {}
impl _cbPluginEvent for cbRightUpEvent {}
impl _wxEvent for cbRightUpEvent {}
impl _wxObject for cbRightUpEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbRightUpEvent {
    pub fn from(handle: *mut c_void) -> @cbRightUpEvent { @cbRightUpEvent(handle) }
    pub fn null() -> @cbRightUpEvent { cbRightUpEvent::from(0 as *mut c_void) }
    
}

pub trait _cbRightUpEvent : _cbPluginEvent {
}

pub struct cbRowDragPlugin(*mut c_void);
impl _cbRowDragPlugin for cbRowDragPlugin {}
impl _cbPluginBase for cbRowDragPlugin {}
impl _wxEvtHandler for cbRowDragPlugin {}
impl _wxObject for cbRowDragPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbRowDragPlugin {
    pub fn from(handle: *mut c_void) -> @cbRowDragPlugin { @cbRowDragPlugin(handle) }
    pub fn null() -> @cbRowDragPlugin { cbRowDragPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbRowDragPlugin : _cbPluginBase {
}

pub struct cbRowInfo(*mut c_void);
impl _cbRowInfo for cbRowInfo {}
impl _wxObject for cbRowInfo { fn handle(&self) -> *mut c_void { **self } }

impl cbRowInfo {
    pub fn from(handle: *mut c_void) -> @cbRowInfo { @cbRowInfo(handle) }
    pub fn null() -> @cbRowInfo { cbRowInfo::from(0 as *mut c_void) }
    
}

pub trait _cbRowInfo : _wxObject {
}

pub struct cbRowLayoutPlugin(*mut c_void);
impl _cbRowLayoutPlugin for cbRowLayoutPlugin {}
impl _cbPluginBase for cbRowLayoutPlugin {}
impl _wxEvtHandler for cbRowLayoutPlugin {}
impl _wxObject for cbRowLayoutPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbRowLayoutPlugin {
    pub fn from(handle: *mut c_void) -> @cbRowLayoutPlugin { @cbRowLayoutPlugin(handle) }
    pub fn null() -> @cbRowLayoutPlugin { cbRowLayoutPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbRowLayoutPlugin : _cbPluginBase {
}

pub struct cbSimpleCustomizationPlugin(*mut c_void);
impl _cbSimpleCustomizationPlugin for cbSimpleCustomizationPlugin {}
impl _cbPluginBase for cbSimpleCustomizationPlugin {}
impl _wxEvtHandler for cbSimpleCustomizationPlugin {}
impl _wxObject for cbSimpleCustomizationPlugin { fn handle(&self) -> *mut c_void { **self } }

impl cbSimpleCustomizationPlugin {
    pub fn from(handle: *mut c_void) -> @cbSimpleCustomizationPlugin { @cbSimpleCustomizationPlugin(handle) }
    pub fn null() -> @cbSimpleCustomizationPlugin { cbSimpleCustomizationPlugin::from(0 as *mut c_void) }
    
}

pub trait _cbSimpleCustomizationPlugin : _cbPluginBase {
}

pub struct cbSimpleUpdatesMgr(*mut c_void);
impl _cbSimpleUpdatesMgr for cbSimpleUpdatesMgr {}
impl _cbUpdatesManagerBase for cbSimpleUpdatesMgr {}
impl _wxObject for cbSimpleUpdatesMgr { fn handle(&self) -> *mut c_void { **self } }

impl cbSimpleUpdatesMgr {
    pub fn from(handle: *mut c_void) -> @cbSimpleUpdatesMgr { @cbSimpleUpdatesMgr(handle) }
    pub fn null() -> @cbSimpleUpdatesMgr { cbSimpleUpdatesMgr::from(0 as *mut c_void) }
    
}

pub trait _cbSimpleUpdatesMgr : _cbUpdatesManagerBase {
}

pub struct cbSizeBarWndEvent(*mut c_void);
impl _cbSizeBarWndEvent for cbSizeBarWndEvent {}
impl _cbPluginEvent for cbSizeBarWndEvent {}
impl _wxEvent for cbSizeBarWndEvent {}
impl _wxObject for cbSizeBarWndEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbSizeBarWndEvent {
    pub fn from(handle: *mut c_void) -> @cbSizeBarWndEvent { @cbSizeBarWndEvent(handle) }
    pub fn null() -> @cbSizeBarWndEvent { cbSizeBarWndEvent::from(0 as *mut c_void) }
    
}

pub trait _cbSizeBarWndEvent : _cbPluginEvent {
}

pub struct cbStartBarDraggingEvent(*mut c_void);
impl _cbStartBarDraggingEvent for cbStartBarDraggingEvent {}
impl _cbPluginEvent for cbStartBarDraggingEvent {}
impl _wxEvent for cbStartBarDraggingEvent {}
impl _wxObject for cbStartBarDraggingEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbStartBarDraggingEvent {
    pub fn from(handle: *mut c_void) -> @cbStartBarDraggingEvent { @cbStartBarDraggingEvent(handle) }
    pub fn null() -> @cbStartBarDraggingEvent { cbStartBarDraggingEvent::from(0 as *mut c_void) }
    
}

pub trait _cbStartBarDraggingEvent : _cbPluginEvent {
}

pub struct cbStartDrawInAreaEvent(*mut c_void);
impl _cbStartDrawInAreaEvent for cbStartDrawInAreaEvent {}
impl _cbPluginEvent for cbStartDrawInAreaEvent {}
impl _wxEvent for cbStartDrawInAreaEvent {}
impl _wxObject for cbStartDrawInAreaEvent { fn handle(&self) -> *mut c_void { **self } }

impl cbStartDrawInAreaEvent {
    pub fn from(handle: *mut c_void) -> @cbStartDrawInAreaEvent { @cbStartDrawInAreaEvent(handle) }
    pub fn null() -> @cbStartDrawInAreaEvent { cbStartDrawInAreaEvent::from(0 as *mut c_void) }
    
}

pub trait _cbStartDrawInAreaEvent : _cbPluginEvent {
}

pub struct cbUpdatesManagerBase(*mut c_void);
impl _cbUpdatesManagerBase for cbUpdatesManagerBase {}
impl _wxObject for cbUpdatesManagerBase { fn handle(&self) -> *mut c_void { **self } }

impl cbUpdatesManagerBase {
    pub fn from(handle: *mut c_void) -> @cbUpdatesManagerBase { @cbUpdatesManagerBase(handle) }
    pub fn null() -> @cbUpdatesManagerBase { cbUpdatesManagerBase::from(0 as *mut c_void) }
    
}

pub trait _cbUpdatesManagerBase : _wxObject {
}
