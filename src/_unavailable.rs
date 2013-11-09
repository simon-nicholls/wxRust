use std::libc::*;
use base::*;
use core::*;

pub struct ELJMessageParameters(*mut c_void);
impl _ELJMessageParameters for ELJMessageParameters { fn handle(&self) -> *mut c_void { **self } }

impl ELJMessageParameters {
    pub fn from(handle: *mut c_void) -> @ELJMessageParameters { @ELJMessageParameters(handle) }
    pub fn null() -> @ELJMessageParameters { ELJMessageParameters::from(0 as *mut c_void) }
    
}

pub trait _ELJMessageParameters {
    fn handle(&self) -> *mut c_void;
    
}

pub struct ELJPlotCurve(*mut c_void);
impl _ELJPlotCurve for ELJPlotCurve {}
impl _wxPlotCurve for ELJPlotCurve {}
impl _wxObject for ELJPlotCurve { fn handle(&self) -> *mut c_void { **self } }

impl ELJPlotCurve {
    pub fn from(handle: *mut c_void) -> @ELJPlotCurve { @ELJPlotCurve(handle) }
    pub fn null() -> @ELJPlotCurve { ELJPlotCurve::from(0 as *mut c_void) }
    
}

pub trait _ELJPlotCurve : _wxPlotCurve {
}

pub struct wxDynToolInfo(*mut c_void);
impl _wxDynToolInfo for wxDynToolInfo {}
impl _wxToolLayoutItem for wxDynToolInfo {}
impl _wxObject for wxDynToolInfo { fn handle(&self) -> *mut c_void { **self } }

impl wxDynToolInfo {
    pub fn from(handle: *mut c_void) -> @wxDynToolInfo { @wxDynToolInfo(handle) }
    pub fn null() -> @wxDynToolInfo { wxDynToolInfo::from(0 as *mut c_void) }
    
}

pub trait _wxDynToolInfo : _wxToolLayoutItem {
}

pub struct wxDynamicSashWindow(*mut c_void);
impl _wxDynamicSashWindow for wxDynamicSashWindow {}
impl _wxWindow for wxDynamicSashWindow {}
impl _wxEvtHandler for wxDynamicSashWindow {}
impl _wxObject for wxDynamicSashWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxDynamicSashWindow {
    pub fn from(handle: *mut c_void) -> @wxDynamicSashWindow { @wxDynamicSashWindow(handle) }
    pub fn null() -> @wxDynamicSashWindow { wxDynamicSashWindow::from(0 as *mut c_void) }
    
}

pub trait _wxDynamicSashWindow : _wxWindow {
}

pub struct wxDynamicToolBar(*mut c_void);
impl _wxDynamicToolBar for wxDynamicToolBar {}
impl _wxToolBarBase for wxDynamicToolBar {}
impl _wxControl for wxDynamicToolBar {}
impl _wxWindow for wxDynamicToolBar {}
impl _wxEvtHandler for wxDynamicToolBar {}
impl _wxObject for wxDynamicToolBar { fn handle(&self) -> *mut c_void { **self } }

impl wxDynamicToolBar {
    pub fn from(handle: *mut c_void) -> @wxDynamicToolBar { @wxDynamicToolBar(handle) }
    pub fn null() -> @wxDynamicToolBar { wxDynamicToolBar::from(0 as *mut c_void) }
    
}

pub trait _wxDynamicToolBar : _wxToolBarBase {
}

pub struct wxExpr(*mut c_void);
impl _wxExpr for wxExpr { fn handle(&self) -> *mut c_void { **self } }

impl wxExpr {
    pub fn from(handle: *mut c_void) -> @wxExpr { @wxExpr(handle) }
    pub fn null() -> @wxExpr { wxExpr::from(0 as *mut c_void) }
    
}

pub trait _wxExpr {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxExprDatabase(*mut c_void);
impl _wxExprDatabase for wxExprDatabase {}
impl _wxList for wxExprDatabase {}
impl _wxObject for wxExprDatabase { fn handle(&self) -> *mut c_void { **self } }

impl wxExprDatabase {
    pub fn from(handle: *mut c_void) -> @wxExprDatabase { @wxExprDatabase(handle) }
    pub fn null() -> @wxExprDatabase { wxExprDatabase::from(0 as *mut c_void) }
    
}

pub trait _wxExprDatabase : _wxList {
}

pub struct wxFrameLayout(*mut c_void);
impl _wxFrameLayout for wxFrameLayout {}
impl _wxEvtHandler for wxFrameLayout {}
impl _wxObject for wxFrameLayout { fn handle(&self) -> *mut c_void { **self } }

impl wxFrameLayout {
    pub fn from(handle: *mut c_void) -> @wxFrameLayout { @wxFrameLayout(handle) }
    pub fn null() -> @wxFrameLayout { wxFrameLayout::from(0 as *mut c_void) }
    
}

pub trait _wxFrameLayout : _wxEvtHandler {
}

pub struct wxHashMap(*mut c_void);
impl _wxHashMap for wxHashMap { fn handle(&self) -> *mut c_void { **self } }

impl wxHashMap {
    pub fn from(handle: *mut c_void) -> @wxHashMap { @wxHashMap(handle) }
    pub fn null() -> @wxHashMap { wxHashMap::from(0 as *mut c_void) }
    
}

pub trait _wxHashMap {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxLEDNumberCtrl(*mut c_void);
impl _wxLEDNumberCtrl for wxLEDNumberCtrl {}
impl _wxControl for wxLEDNumberCtrl {}
impl _wxWindow for wxLEDNumberCtrl {}
impl _wxEvtHandler for wxLEDNumberCtrl {}
impl _wxObject for wxLEDNumberCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxLEDNumberCtrl {
    pub fn from(handle: *mut c_void) -> @wxLEDNumberCtrl { @wxLEDNumberCtrl(handle) }
    pub fn null() -> @wxLEDNumberCtrl { wxLEDNumberCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxLEDNumberCtrl : _wxControl {
}

pub struct wxMBConvFile(*mut c_void);
impl _wxMBConvFile for wxMBConvFile {}
impl _wxMBConv for wxMBConvFile { fn handle(&self) -> *mut c_void { **self } }

impl wxMBConvFile {
    pub fn from(handle: *mut c_void) -> @wxMBConvFile { @wxMBConvFile(handle) }
    pub fn null() -> @wxMBConvFile { wxMBConvFile::from(0 as *mut c_void) }
    
}

pub trait _wxMBConvFile : _wxMBConv {
}

pub struct wxMultiCellCanvas(*mut c_void);
impl _wxMultiCellCanvas for wxMultiCellCanvas {}
impl _wxFlexGridSizer for wxMultiCellCanvas {}
impl _wxGridSizer for wxMultiCellCanvas {}
impl _wxSizer for wxMultiCellCanvas {}
impl _wxObject for wxMultiCellCanvas { fn handle(&self) -> *mut c_void { **self } }

impl wxMultiCellCanvas {
    pub fn from(handle: *mut c_void) -> @wxMultiCellCanvas { @wxMultiCellCanvas(handle) }
    pub fn null() -> @wxMultiCellCanvas { wxMultiCellCanvas::from(0 as *mut c_void) }
    
}

pub trait _wxMultiCellCanvas : _wxFlexGridSizer {
}

pub struct wxMultiCellItemHandle(*mut c_void);
impl _wxMultiCellItemHandle for wxMultiCellItemHandle {}
impl _wxObject for wxMultiCellItemHandle { fn handle(&self) -> *mut c_void { **self } }

impl wxMultiCellItemHandle {
    pub fn from(handle: *mut c_void) -> @wxMultiCellItemHandle { @wxMultiCellItemHandle(handle) }
    pub fn null() -> @wxMultiCellItemHandle { wxMultiCellItemHandle::from(0 as *mut c_void) }
    
}

pub trait _wxMultiCellItemHandle : _wxObject {
}

pub struct wxMultiCellSizer(*mut c_void);
impl _wxMultiCellSizer for wxMultiCellSizer {}
impl _wxSizer for wxMultiCellSizer {}
impl _wxObject for wxMultiCellSizer { fn handle(&self) -> *mut c_void { **self } }

impl wxMultiCellSizer {
    pub fn from(handle: *mut c_void) -> @wxMultiCellSizer { @wxMultiCellSizer(handle) }
    pub fn null() -> @wxMultiCellSizer { wxMultiCellSizer::from(0 as *mut c_void) }
    
}

pub trait _wxMultiCellSizer : _wxSizer {
}

pub struct wxNewBitmapButton(*mut c_void);
impl _wxNewBitmapButton for wxNewBitmapButton {}
impl _wxPanel for wxNewBitmapButton {}
impl _wxWindow for wxNewBitmapButton {}
impl _wxEvtHandler for wxNewBitmapButton {}
impl _wxObject for wxNewBitmapButton { fn handle(&self) -> *mut c_void { **self } }

impl wxNewBitmapButton {
    pub fn from(handle: *mut c_void) -> @wxNewBitmapButton { @wxNewBitmapButton(handle) }
    pub fn null() -> @wxNewBitmapButton { wxNewBitmapButton::from(0 as *mut c_void) }
    
}

pub trait _wxNewBitmapButton : _wxPanel {
}

pub struct wxPlotCurve(*mut c_void);
impl _wxPlotCurve for wxPlotCurve {}
impl _wxObject for wxPlotCurve { fn handle(&self) -> *mut c_void { **self } }

impl wxPlotCurve {
    pub fn from(handle: *mut c_void) -> @wxPlotCurve { @wxPlotCurve(handle) }
    pub fn null() -> @wxPlotCurve { wxPlotCurve::from(0 as *mut c_void) }
    
}

pub trait _wxPlotCurve : _wxObject {
}

pub struct wxPlotEvent(*mut c_void);
impl _wxPlotEvent for wxPlotEvent {}
impl _wxNotifyEvent for wxPlotEvent {}
impl _wxCommandEvent for wxPlotEvent {}
impl _wxEvent for wxPlotEvent {}
impl _wxObject for wxPlotEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxPlotEvent {
    pub fn from(handle: *mut c_void) -> @wxPlotEvent { @wxPlotEvent(handle) }
    pub fn null() -> @wxPlotEvent { wxPlotEvent::from(0 as *mut c_void) }
    
}

pub trait _wxPlotEvent : _wxNotifyEvent {
}

pub struct wxPlotOnOffCurve(*mut c_void);
impl _wxPlotOnOffCurve for wxPlotOnOffCurve {}
impl _wxObject for wxPlotOnOffCurve { fn handle(&self) -> *mut c_void { **self } }

impl wxPlotOnOffCurve {
    pub fn from(handle: *mut c_void) -> @wxPlotOnOffCurve { @wxPlotOnOffCurve(handle) }
    pub fn null() -> @wxPlotOnOffCurve { wxPlotOnOffCurve::from(0 as *mut c_void) }
    
}

pub trait _wxPlotOnOffCurve : _wxObject {
}

pub struct wxPlotWindow(*mut c_void);
impl _wxPlotWindow for wxPlotWindow {}
impl _wxScrolledWindow for wxPlotWindow {}
impl _wxPanel for wxPlotWindow {}
impl _wxWindow for wxPlotWindow {}
impl _wxEvtHandler for wxPlotWindow {}
impl _wxObject for wxPlotWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxPlotWindow {
    pub fn from(handle: *mut c_void) -> @wxPlotWindow { @wxPlotWindow(handle) }
    pub fn null() -> @wxPlotWindow { wxPlotWindow::from(0 as *mut c_void) }
    
}

pub trait _wxPlotWindow : _wxScrolledWindow {
}

pub struct wxRemotelyScrolledTreeCtrl(*mut c_void);
impl _wxRemotelyScrolledTreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl _wxTreeCtrl for wxRemotelyScrolledTreeCtrl {}
impl _wxControl for wxRemotelyScrolledTreeCtrl {}
impl _wxWindow for wxRemotelyScrolledTreeCtrl {}
impl _wxEvtHandler for wxRemotelyScrolledTreeCtrl {}
impl _wxObject for wxRemotelyScrolledTreeCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxRemotelyScrolledTreeCtrl {
    pub fn from(handle: *mut c_void) -> @wxRemotelyScrolledTreeCtrl { @wxRemotelyScrolledTreeCtrl(handle) }
    pub fn null() -> @wxRemotelyScrolledTreeCtrl { wxRemotelyScrolledTreeCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxRemotelyScrolledTreeCtrl : _wxTreeCtrl {
}

pub struct wxSplitterScrolledWindow(*mut c_void);
impl _wxSplitterScrolledWindow for wxSplitterScrolledWindow {}
impl _wxScrolledWindow for wxSplitterScrolledWindow {}
impl _wxPanel for wxSplitterScrolledWindow {}
impl _wxWindow for wxSplitterScrolledWindow {}
impl _wxEvtHandler for wxSplitterScrolledWindow {}
impl _wxObject for wxSplitterScrolledWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxSplitterScrolledWindow {
    pub fn from(handle: *mut c_void) -> @wxSplitterScrolledWindow { @wxSplitterScrolledWindow(handle) }
    pub fn null() -> @wxSplitterScrolledWindow { wxSplitterScrolledWindow::from(0 as *mut c_void) }
    
}

pub trait _wxSplitterScrolledWindow : _wxScrolledWindow {
}

pub struct wxStreamToTextRedirector(*mut c_void);
impl _wxStreamToTextRedirector for wxStreamToTextRedirector { fn handle(&self) -> *mut c_void { **self } }

impl wxStreamToTextRedirector {
    pub fn from(handle: *mut c_void) -> @wxStreamToTextRedirector { @wxStreamToTextRedirector(handle) }
    pub fn null() -> @wxStreamToTextRedirector { wxStreamToTextRedirector::from(0 as *mut c_void) }
    
}

pub trait _wxStreamToTextRedirector {
    fn handle(&self) -> *mut c_void;
    
}

pub struct wxTabCtrl(*mut c_void);
impl _wxTabCtrl for wxTabCtrl {}
impl _wxControl for wxTabCtrl {}
impl _wxWindow for wxTabCtrl {}
impl _wxEvtHandler for wxTabCtrl {}
impl _wxObject for wxTabCtrl { fn handle(&self) -> *mut c_void { **self } }

impl wxTabCtrl {
    pub fn from(handle: *mut c_void) -> @wxTabCtrl { @wxTabCtrl(handle) }
    pub fn null() -> @wxTabCtrl { wxTabCtrl::from(0 as *mut c_void) }
    
}

pub trait _wxTabCtrl : _wxControl {
}

pub struct wxTabEvent(*mut c_void);
impl _wxTabEvent for wxTabEvent {}
impl _wxCommandEvent for wxTabEvent {}
impl _wxEvent for wxTabEvent {}
impl _wxObject for wxTabEvent { fn handle(&self) -> *mut c_void { **self } }

impl wxTabEvent {
    pub fn from(handle: *mut c_void) -> @wxTabEvent { @wxTabEvent(handle) }
    pub fn null() -> @wxTabEvent { wxTabEvent::from(0 as *mut c_void) }
    
}

pub trait _wxTabEvent : _wxCommandEvent {
}

pub struct wxThinSplitterWindow(*mut c_void);
impl _wxThinSplitterWindow for wxThinSplitterWindow {}
impl _wxSplitterWindow for wxThinSplitterWindow {}
impl _wxWindow for wxThinSplitterWindow {}
impl _wxEvtHandler for wxThinSplitterWindow {}
impl _wxObject for wxThinSplitterWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxThinSplitterWindow {
    pub fn from(handle: *mut c_void) -> @wxThinSplitterWindow { @wxThinSplitterWindow(handle) }
    pub fn null() -> @wxThinSplitterWindow { wxThinSplitterWindow::from(0 as *mut c_void) }
    
}

pub trait _wxThinSplitterWindow : _wxSplitterWindow {
}

pub struct wxTimerBase(*mut c_void);
impl _wxTimerBase for wxTimerBase {}
impl _wxObject for wxTimerBase { fn handle(&self) -> *mut c_void { **self } }

impl wxTimerBase {
    pub fn from(handle: *mut c_void) -> @wxTimerBase { @wxTimerBase(handle) }
    pub fn null() -> @wxTimerBase { wxTimerBase::from(0 as *mut c_void) }
    
}

pub trait _wxTimerBase : _wxObject {
}

pub struct wxToolLayoutItem(*mut c_void);
impl _wxToolLayoutItem for wxToolLayoutItem {}
impl _wxObject for wxToolLayoutItem { fn handle(&self) -> *mut c_void { **self } }

impl wxToolLayoutItem {
    pub fn from(handle: *mut c_void) -> @wxToolLayoutItem { @wxToolLayoutItem(handle) }
    pub fn null() -> @wxToolLayoutItem { wxToolLayoutItem::from(0 as *mut c_void) }
    
}

pub trait _wxToolLayoutItem : _wxObject {
}

pub struct wxToolWindow(*mut c_void);
impl _wxToolWindow for wxToolWindow {}
impl _wxFrame for wxToolWindow {}
impl _wxTopLevelWindow for wxToolWindow {}
impl _wxWindow for wxToolWindow {}
impl _wxEvtHandler for wxToolWindow {}
impl _wxObject for wxToolWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxToolWindow {
    pub fn from(handle: *mut c_void) -> @wxToolWindow { @wxToolWindow(handle) }
    pub fn null() -> @wxToolWindow { wxToolWindow::from(0 as *mut c_void) }
    
}

pub trait _wxToolWindow : _wxFrame {
}

pub struct wxTreeCompanionWindow(*mut c_void);
impl _wxTreeCompanionWindow for wxTreeCompanionWindow {}
impl _wxWindow for wxTreeCompanionWindow {}
impl _wxEvtHandler for wxTreeCompanionWindow {}
impl _wxObject for wxTreeCompanionWindow { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeCompanionWindow {
    pub fn from(handle: *mut c_void) -> @wxTreeCompanionWindow { @wxTreeCompanionWindow(handle) }
    pub fn null() -> @wxTreeCompanionWindow { wxTreeCompanionWindow::from(0 as *mut c_void) }
    
}

pub trait _wxTreeCompanionWindow : _wxWindow {
}

pub struct wxTreeLayout(*mut c_void);
impl _wxTreeLayout for wxTreeLayout {}
impl _wxObject for wxTreeLayout { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeLayout {
    pub fn from(handle: *mut c_void) -> @wxTreeLayout { @wxTreeLayout(handle) }
    pub fn null() -> @wxTreeLayout { wxTreeLayout::from(0 as *mut c_void) }
    
}

pub trait _wxTreeLayout : _wxObject {
}

pub struct wxTreeLayoutStored(*mut c_void);
impl _wxTreeLayoutStored for wxTreeLayoutStored {}
impl _wxTreeLayout for wxTreeLayoutStored {}
impl _wxObject for wxTreeLayoutStored { fn handle(&self) -> *mut c_void { **self } }

impl wxTreeLayoutStored {
    pub fn from(handle: *mut c_void) -> @wxTreeLayoutStored { @wxTreeLayoutStored(handle) }
    pub fn null() -> @wxTreeLayoutStored { wxTreeLayoutStored::from(0 as *mut c_void) }
    
}

pub trait _wxTreeLayoutStored : _wxTreeLayout {
}

pub struct wxGauge95(*mut c_void);
impl _wxGauge95 for wxGauge95 {}
impl _wxGauge for wxGauge95 {}
impl _wxControl for wxGauge95 {}
impl _wxWindow for wxGauge95 {}
impl _wxEvtHandler for wxGauge95 {}
impl _wxObject for wxGauge95 { fn handle(&self) -> *mut c_void { **self } }

impl wxGauge95 {
    pub fn from(handle: *mut c_void) -> @wxGauge95 { @wxGauge95(handle) }
    pub fn null() -> @wxGauge95 { wxGauge95::from(0 as *mut c_void) }
    
}

pub trait _wxGauge95 : _wxGauge {
}

pub struct wxGaugeMSW(*mut c_void);
impl _wxGaugeMSW for wxGaugeMSW {}
impl _wxGauge for wxGaugeMSW {}
impl _wxControl for wxGaugeMSW {}
impl _wxWindow for wxGaugeMSW {}
impl _wxEvtHandler for wxGaugeMSW {}
impl _wxObject for wxGaugeMSW { fn handle(&self) -> *mut c_void { **self } }

impl wxGaugeMSW {
    pub fn from(handle: *mut c_void) -> @wxGaugeMSW { @wxGaugeMSW(handle) }
    pub fn null() -> @wxGaugeMSW { wxGaugeMSW::from(0 as *mut c_void) }
    
}

pub trait _wxGaugeMSW : _wxGauge {
}

pub struct wxSlider95(*mut c_void);
impl _wxSlider95 for wxSlider95 {}
impl _wxSlider for wxSlider95 {}
impl _wxControl for wxSlider95 {}
impl _wxWindow for wxSlider95 {}
impl _wxEvtHandler for wxSlider95 {}
impl _wxObject for wxSlider95 { fn handle(&self) -> *mut c_void { **self } }

impl wxSlider95 {
    pub fn from(handle: *mut c_void) -> @wxSlider95 { @wxSlider95(handle) }
    pub fn null() -> @wxSlider95 { wxSlider95::from(0 as *mut c_void) }
    
}

pub trait _wxSlider95 : _wxSlider {
}

pub struct wxSliderMSW(*mut c_void);
impl _wxSliderMSW for wxSliderMSW {}
impl _wxSlider for wxSliderMSW {}
impl _wxControl for wxSliderMSW {}
impl _wxWindow for wxSliderMSW {}
impl _wxEvtHandler for wxSliderMSW {}
impl _wxObject for wxSliderMSW { fn handle(&self) -> *mut c_void { **self } }

impl wxSliderMSW {
    pub fn from(handle: *mut c_void) -> @wxSliderMSW { @wxSliderMSW(handle) }
    pub fn null() -> @wxSliderMSW { wxSliderMSW::from(0 as *mut c_void) }
    
}

pub trait _wxSliderMSW : _wxSlider {
}
