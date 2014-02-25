use std::libc::*;
use base::*;
use core::*;

pub struct RustMessageParameters { ptr: *mut c_void }
impl TRustMessageParameters for RustMessageParameters { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustMessageParameters {
    pub fn from(ptr: *mut c_void) -> RustMessageParameters { RustMessageParameters { ptr: ptr } }
    pub fn null() -> RustMessageParameters { RustMessageParameters::from(0 as *mut c_void) }
    
}

pub trait TRustMessageParameters {
    fn ptr(&self) -> *mut c_void;
    
}

/// The wxRust-specific derived class of [wxPlotCurve](http://docs.wxwidgets.org/3.0/classwx_plot_curve.html).
pub struct RustPlotCurve { ptr: *mut c_void }
impl TRustPlotCurve for RustPlotCurve {}
impl TPlotCurve for RustPlotCurve {}
impl TObject for RustPlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RustPlotCurve {
    pub fn from(ptr: *mut c_void) -> RustPlotCurve { RustPlotCurve { ptr: ptr } }
    pub fn null() -> RustPlotCurve { RustPlotCurve::from(0 as *mut c_void) }
    
}

/// Methods of the wxRust-specific derived class of [wxPlotCurve](http://docs.wxwidgets.org/3.0/classwx_plot_curve.html).
pub trait TRustPlotCurve : TPlotCurve {
}

/// Wraps the wxWidgets' [wxDynToolInfo](http://docs.wxwidgets.org/3.0/classwx_dyn_tool_info.html) class.
pub struct DynToolInfo { ptr: *mut c_void }
impl TDynToolInfo for DynToolInfo {}
impl TToolLayoutItem for DynToolInfo {}
impl TObject for DynToolInfo { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynToolInfo {
    pub fn from(ptr: *mut c_void) -> DynToolInfo { DynToolInfo { ptr: ptr } }
    pub fn null() -> DynToolInfo { DynToolInfo::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDynToolInfo](http://docs.wxwidgets.org/3.0/classwx_dyn_tool_info.html) class.
pub trait TDynToolInfo : TToolLayoutItem {
}

/// Wraps the wxWidgets' [wxDynamicSashWindow](http://docs.wxwidgets.org/3.0/classwx_dynamic_sash_window.html) class.
pub struct DynamicSashWindow { ptr: *mut c_void }
impl TDynamicSashWindow for DynamicSashWindow {}
impl TWindow for DynamicSashWindow {}
impl TEvtHandler for DynamicSashWindow {}
impl TObject for DynamicSashWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynamicSashWindow {
    pub fn from(ptr: *mut c_void) -> DynamicSashWindow { DynamicSashWindow { ptr: ptr } }
    pub fn null() -> DynamicSashWindow { DynamicSashWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDynamicSashWindow](http://docs.wxwidgets.org/3.0/classwx_dynamic_sash_window.html) class.
pub trait TDynamicSashWindow : TWindow {
}

/// Wraps the wxWidgets' [wxDynamicToolBar](http://docs.wxwidgets.org/3.0/classwx_dynamic_tool_bar.html) class.
pub struct DynamicToolBar { ptr: *mut c_void }
impl TDynamicToolBar for DynamicToolBar {}
impl TToolBarBase for DynamicToolBar {}
impl TControl for DynamicToolBar {}
impl TWindow for DynamicToolBar {}
impl TEvtHandler for DynamicToolBar {}
impl TObject for DynamicToolBar { fn ptr(&self) -> *mut c_void { self.ptr } }

impl DynamicToolBar {
    pub fn from(ptr: *mut c_void) -> DynamicToolBar { DynamicToolBar { ptr: ptr } }
    pub fn null() -> DynamicToolBar { DynamicToolBar::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxDynamicToolBar](http://docs.wxwidgets.org/3.0/classwx_dynamic_tool_bar.html) class.
pub trait TDynamicToolBar : TToolBarBase {
}

/// Wraps the wxWidgets' [wxExpr](http://docs.wxwidgets.org/3.0/classwx_expr.html) class.
pub struct Expr { ptr: *mut c_void }
impl TExpr for Expr { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Expr {
    pub fn from(ptr: *mut c_void) -> Expr { Expr { ptr: ptr } }
    pub fn null() -> Expr { Expr::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxExpr](http://docs.wxwidgets.org/3.0/classwx_expr.html) class.
pub trait TExpr {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxExprDatabase](http://docs.wxwidgets.org/3.0/classwx_expr_database.html) class.
pub struct ExprDatabase { ptr: *mut c_void }
impl TExprDatabase for ExprDatabase {}
impl TList for ExprDatabase {}
impl TObject for ExprDatabase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ExprDatabase {
    pub fn from(ptr: *mut c_void) -> ExprDatabase { ExprDatabase { ptr: ptr } }
    pub fn null() -> ExprDatabase { ExprDatabase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxExprDatabase](http://docs.wxwidgets.org/3.0/classwx_expr_database.html) class.
pub trait TExprDatabase : TList {
}

/// Wraps the wxWidgets' [wxFrameLayout](http://docs.wxwidgets.org/3.0/classwx_frame_layout.html) class.
pub struct FrameLayout { ptr: *mut c_void }
impl TFrameLayout for FrameLayout {}
impl TEvtHandler for FrameLayout {}
impl TObject for FrameLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl FrameLayout {
    pub fn from(ptr: *mut c_void) -> FrameLayout { FrameLayout { ptr: ptr } }
    pub fn null() -> FrameLayout { FrameLayout::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxFrameLayout](http://docs.wxwidgets.org/3.0/classwx_frame_layout.html) class.
pub trait TFrameLayout : TEvtHandler {
}

/// Wraps the wxWidgets' [wxHashMap](http://docs.wxwidgets.org/3.0/classwx_hash_map.html) class.
pub struct HashMap { ptr: *mut c_void }
impl THashMap for HashMap { fn ptr(&self) -> *mut c_void { self.ptr } }

impl HashMap {
    pub fn from(ptr: *mut c_void) -> HashMap { HashMap { ptr: ptr } }
    pub fn null() -> HashMap { HashMap::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxHashMap](http://docs.wxwidgets.org/3.0/classwx_hash_map.html) class.
pub trait THashMap {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxLEDNumberCtrl](http://docs.wxwidgets.org/3.0/classwx_ledn_umber_ctrl.html) class.
pub struct LEDNumberCtrl { ptr: *mut c_void }
impl TLEDNumberCtrl for LEDNumberCtrl {}
impl TControl for LEDNumberCtrl {}
impl TWindow for LEDNumberCtrl {}
impl TEvtHandler for LEDNumberCtrl {}
impl TObject for LEDNumberCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl LEDNumberCtrl {
    pub fn from(ptr: *mut c_void) -> LEDNumberCtrl { LEDNumberCtrl { ptr: ptr } }
    pub fn null() -> LEDNumberCtrl { LEDNumberCtrl::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxLEDNumberCtrl](http://docs.wxwidgets.org/3.0/classwx_ledn_umber_ctrl.html) class.
pub trait TLEDNumberCtrl : TControl {
}

/// Wraps the wxWidgets' [wxMBConvFile](http://docs.wxwidgets.org/3.0/classwx_mbc_onv_file.html) class.
pub struct MBConvFile { ptr: *mut c_void }
impl TMBConvFile for MBConvFile {}
impl TMBConv for MBConvFile { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MBConvFile {
    pub fn from(ptr: *mut c_void) -> MBConvFile { MBConvFile { ptr: ptr } }
    pub fn null() -> MBConvFile { MBConvFile::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMBConvFile](http://docs.wxwidgets.org/3.0/classwx_mbc_onv_file.html) class.
pub trait TMBConvFile : TMBConv {
}

/// Wraps the wxWidgets' [wxMultiCellCanvas](http://docs.wxwidgets.org/3.0/classwx_multi_cell_canvas.html) class.
pub struct MultiCellCanvas { ptr: *mut c_void }
impl TMultiCellCanvas for MultiCellCanvas {}
impl TFlexGridSizer for MultiCellCanvas {}
impl TGridSizer for MultiCellCanvas {}
impl TSizer for MultiCellCanvas {}
impl TObject for MultiCellCanvas { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MultiCellCanvas {
    pub fn from(ptr: *mut c_void) -> MultiCellCanvas { MultiCellCanvas { ptr: ptr } }
    pub fn null() -> MultiCellCanvas { MultiCellCanvas::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMultiCellCanvas](http://docs.wxwidgets.org/3.0/classwx_multi_cell_canvas.html) class.
pub trait TMultiCellCanvas : TFlexGridSizer {
}

/// Wraps the wxWidgets' [wxMultiCellItemHandle](http://docs.wxwidgets.org/3.0/classwx_multi_cell_item_handle.html) class.
pub struct MultiCellItemHandle { ptr: *mut c_void }
impl TMultiCellItemHandle for MultiCellItemHandle {}
impl TObject for MultiCellItemHandle { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MultiCellItemHandle {
    pub fn from(ptr: *mut c_void) -> MultiCellItemHandle { MultiCellItemHandle { ptr: ptr } }
    pub fn null() -> MultiCellItemHandle { MultiCellItemHandle::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMultiCellItemHandle](http://docs.wxwidgets.org/3.0/classwx_multi_cell_item_handle.html) class.
pub trait TMultiCellItemHandle : TObject {
}

/// Wraps the wxWidgets' [wxMultiCellSizer](http://docs.wxwidgets.org/3.0/classwx_multi_cell_sizer.html) class.
pub struct MultiCellSizer { ptr: *mut c_void }
impl TMultiCellSizer for MultiCellSizer {}
impl TSizer for MultiCellSizer {}
impl TObject for MultiCellSizer { fn ptr(&self) -> *mut c_void { self.ptr } }

impl MultiCellSizer {
    pub fn from(ptr: *mut c_void) -> MultiCellSizer { MultiCellSizer { ptr: ptr } }
    pub fn null() -> MultiCellSizer { MultiCellSizer::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxMultiCellSizer](http://docs.wxwidgets.org/3.0/classwx_multi_cell_sizer.html) class.
pub trait TMultiCellSizer : TSizer {
}

/// Wraps the wxWidgets' [wxNewBitmapButton](http://docs.wxwidgets.org/3.0/classwx_new_bitmap_button.html) class.
pub struct NewBitmapButton { ptr: *mut c_void }
impl TNewBitmapButton for NewBitmapButton {}
impl TPanel for NewBitmapButton {}
impl TWindow for NewBitmapButton {}
impl TEvtHandler for NewBitmapButton {}
impl TObject for NewBitmapButton { fn ptr(&self) -> *mut c_void { self.ptr } }

impl NewBitmapButton {
    pub fn from(ptr: *mut c_void) -> NewBitmapButton { NewBitmapButton { ptr: ptr } }
    pub fn null() -> NewBitmapButton { NewBitmapButton::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxNewBitmapButton](http://docs.wxwidgets.org/3.0/classwx_new_bitmap_button.html) class.
pub trait TNewBitmapButton : TPanel {
}

/// Wraps the wxWidgets' [wxPlotCurve](http://docs.wxwidgets.org/3.0/classwx_plot_curve.html) class.
/// Rather use the wxRust-specific [RustPlotCurve](struct.RustPlotCurve.html) class.
pub struct PlotCurve { ptr: *mut c_void }
impl TPlotCurve for PlotCurve {}
impl TObject for PlotCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotCurve {
    pub fn from(ptr: *mut c_void) -> PlotCurve { PlotCurve { ptr: ptr } }
    pub fn null() -> PlotCurve { PlotCurve::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPlotCurve](http://docs.wxwidgets.org/3.0/classwx_plot_curve.html) class.
pub trait TPlotCurve : TObject {
}

/// Wraps the wxWidgets' [wxPlotEvent](http://docs.wxwidgets.org/3.0/classwx_plot_event.html) class.
pub struct PlotEvent { ptr: *mut c_void }
impl TPlotEvent for PlotEvent {}
impl TNotifyEvent for PlotEvent {}
impl TCommandEvent for PlotEvent {}
impl TEvent for PlotEvent {}
impl TObject for PlotEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotEvent {
    pub fn from(ptr: *mut c_void) -> PlotEvent { PlotEvent { ptr: ptr } }
    pub fn null() -> PlotEvent { PlotEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPlotEvent](http://docs.wxwidgets.org/3.0/classwx_plot_event.html) class.
pub trait TPlotEvent : TNotifyEvent {
}

/// Wraps the wxWidgets' [wxPlotOnOffCurve](http://docs.wxwidgets.org/3.0/classwx_plot_on_off_curve.html) class.
pub struct PlotOnOffCurve { ptr: *mut c_void }
impl TPlotOnOffCurve for PlotOnOffCurve {}
impl TObject for PlotOnOffCurve { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotOnOffCurve {
    pub fn from(ptr: *mut c_void) -> PlotOnOffCurve { PlotOnOffCurve { ptr: ptr } }
    pub fn null() -> PlotOnOffCurve { PlotOnOffCurve::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPlotOnOffCurve](http://docs.wxwidgets.org/3.0/classwx_plot_on_off_curve.html) class.
pub trait TPlotOnOffCurve : TObject {
}

/// Wraps the wxWidgets' [wxPlotWindow](http://docs.wxwidgets.org/3.0/classwx_plot_window.html) class.
pub struct PlotWindow { ptr: *mut c_void }
impl TPlotWindow for PlotWindow {}
impl TScrolledWindow for PlotWindow {}
impl TPanel for PlotWindow {}
impl TWindow for PlotWindow {}
impl TEvtHandler for PlotWindow {}
impl TObject for PlotWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl PlotWindow {
    pub fn from(ptr: *mut c_void) -> PlotWindow { PlotWindow { ptr: ptr } }
    pub fn null() -> PlotWindow { PlotWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxPlotWindow](http://docs.wxwidgets.org/3.0/classwx_plot_window.html) class.
pub trait TPlotWindow : TScrolledWindow {
}

/// Wraps the wxWidgets' [wxRemotelyScrolledTreeCtrl](http://docs.wxwidgets.org/3.0/classwx_remotely_scrolled_tree_ctrl.html) class.
pub struct RemotelyScrolledTreeCtrl { ptr: *mut c_void }
impl TRemotelyScrolledTreeCtrl for RemotelyScrolledTreeCtrl {}
impl TTreeCtrl for RemotelyScrolledTreeCtrl {}
impl TControl for RemotelyScrolledTreeCtrl {}
impl TWindow for RemotelyScrolledTreeCtrl {}
impl TEvtHandler for RemotelyScrolledTreeCtrl {}
impl TObject for RemotelyScrolledTreeCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl RemotelyScrolledTreeCtrl {
    pub fn from(ptr: *mut c_void) -> RemotelyScrolledTreeCtrl { RemotelyScrolledTreeCtrl { ptr: ptr } }
    pub fn null() -> RemotelyScrolledTreeCtrl { RemotelyScrolledTreeCtrl::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxRemotelyScrolledTreeCtrl](http://docs.wxwidgets.org/3.0/classwx_remotely_scrolled_tree_ctrl.html) class.
pub trait TRemotelyScrolledTreeCtrl : TTreeCtrl {
}

/// Wraps the wxWidgets' [wxSplitterScrolledWindow](http://docs.wxwidgets.org/3.0/classwx_splitter_scrolled_window.html) class.
pub struct SplitterScrolledWindow { ptr: *mut c_void }
impl TSplitterScrolledWindow for SplitterScrolledWindow {}
impl TScrolledWindow for SplitterScrolledWindow {}
impl TPanel for SplitterScrolledWindow {}
impl TWindow for SplitterScrolledWindow {}
impl TEvtHandler for SplitterScrolledWindow {}
impl TObject for SplitterScrolledWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SplitterScrolledWindow {
    pub fn from(ptr: *mut c_void) -> SplitterScrolledWindow { SplitterScrolledWindow { ptr: ptr } }
    pub fn null() -> SplitterScrolledWindow { SplitterScrolledWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSplitterScrolledWindow](http://docs.wxwidgets.org/3.0/classwx_splitter_scrolled_window.html) class.
pub trait TSplitterScrolledWindow : TScrolledWindow {
}

/// Wraps the wxWidgets' [wxStreamToTextRedirector](http://docs.wxwidgets.org/3.0/classwx_stream_to_text_redirector.html) class.
pub struct StreamToTextRedirector { ptr: *mut c_void }
impl TStreamToTextRedirector for StreamToTextRedirector { fn ptr(&self) -> *mut c_void { self.ptr } }

impl StreamToTextRedirector {
    pub fn from(ptr: *mut c_void) -> StreamToTextRedirector { StreamToTextRedirector { ptr: ptr } }
    pub fn null() -> StreamToTextRedirector { StreamToTextRedirector::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxStreamToTextRedirector](http://docs.wxwidgets.org/3.0/classwx_stream_to_text_redirector.html) class.
pub trait TStreamToTextRedirector {
    fn ptr(&self) -> *mut c_void;
    
}

/// Wraps the wxWidgets' [wxTabCtrl](http://docs.wxwidgets.org/3.0/classwx_tab_ctrl.html) class.
pub struct TabCtrl { ptr: *mut c_void }
impl TTabCtrl for TabCtrl {}
impl TControl for TabCtrl {}
impl TWindow for TabCtrl {}
impl TEvtHandler for TabCtrl {}
impl TObject for TabCtrl { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TabCtrl {
    pub fn from(ptr: *mut c_void) -> TabCtrl { TabCtrl { ptr: ptr } }
    pub fn null() -> TabCtrl { TabCtrl::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTabCtrl](http://docs.wxwidgets.org/3.0/classwx_tab_ctrl.html) class.
pub trait TTabCtrl : TControl {
}

/// Wraps the wxWidgets' [wxTabEvent](http://docs.wxwidgets.org/3.0/classwx_tab_event.html) class.
pub struct TabEvent { ptr: *mut c_void }
impl TTabEvent for TabEvent {}
impl TCommandEvent for TabEvent {}
impl TEvent for TabEvent {}
impl TObject for TabEvent { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TabEvent {
    pub fn from(ptr: *mut c_void) -> TabEvent { TabEvent { ptr: ptr } }
    pub fn null() -> TabEvent { TabEvent::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTabEvent](http://docs.wxwidgets.org/3.0/classwx_tab_event.html) class.
pub trait TTabEvent : TCommandEvent {
}

/// Wraps the wxWidgets' [wxThinSplitterWindow](http://docs.wxwidgets.org/3.0/classwx_thin_splitter_window.html) class.
pub struct ThinSplitterWindow { ptr: *mut c_void }
impl TThinSplitterWindow for ThinSplitterWindow {}
impl TSplitterWindow for ThinSplitterWindow {}
impl TWindow for ThinSplitterWindow {}
impl TEvtHandler for ThinSplitterWindow {}
impl TObject for ThinSplitterWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ThinSplitterWindow {
    pub fn from(ptr: *mut c_void) -> ThinSplitterWindow { ThinSplitterWindow { ptr: ptr } }
    pub fn null() -> ThinSplitterWindow { ThinSplitterWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxThinSplitterWindow](http://docs.wxwidgets.org/3.0/classwx_thin_splitter_window.html) class.
pub trait TThinSplitterWindow : TSplitterWindow {
}

/// Wraps the wxWidgets' [wxTimerBase](http://docs.wxwidgets.org/3.0/classwx_timer_base.html) class.
pub struct TimerBase { ptr: *mut c_void }
impl TTimerBase for TimerBase {}
impl TObject for TimerBase { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TimerBase {
    pub fn from(ptr: *mut c_void) -> TimerBase { TimerBase { ptr: ptr } }
    pub fn null() -> TimerBase { TimerBase::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTimerBase](http://docs.wxwidgets.org/3.0/classwx_timer_base.html) class.
pub trait TTimerBase : TObject {
}

/// Wraps the wxWidgets' [wxToolLayoutItem](http://docs.wxwidgets.org/3.0/classwx_tool_layout_item.html) class.
pub struct ToolLayoutItem { ptr: *mut c_void }
impl TToolLayoutItem for ToolLayoutItem {}
impl TObject for ToolLayoutItem { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolLayoutItem {
    pub fn from(ptr: *mut c_void) -> ToolLayoutItem { ToolLayoutItem { ptr: ptr } }
    pub fn null() -> ToolLayoutItem { ToolLayoutItem::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxToolLayoutItem](http://docs.wxwidgets.org/3.0/classwx_tool_layout_item.html) class.
pub trait TToolLayoutItem : TObject {
}

/// Wraps the wxWidgets' [wxToolWindow](http://docs.wxwidgets.org/3.0/classwx_tool_window.html) class.
pub struct ToolWindow { ptr: *mut c_void }
impl TToolWindow for ToolWindow {}
impl TFrame for ToolWindow {}
impl TTopLevelWindow for ToolWindow {}
impl TWindow for ToolWindow {}
impl TEvtHandler for ToolWindow {}
impl TObject for ToolWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl ToolWindow {
    pub fn from(ptr: *mut c_void) -> ToolWindow { ToolWindow { ptr: ptr } }
    pub fn null() -> ToolWindow { ToolWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxToolWindow](http://docs.wxwidgets.org/3.0/classwx_tool_window.html) class.
pub trait TToolWindow : TFrame {
}

/// Wraps the wxWidgets' [wxTreeCompanionWindow](http://docs.wxwidgets.org/3.0/classwx_tree_companion_window.html) class.
pub struct TreeCompanionWindow { ptr: *mut c_void }
impl TTreeCompanionWindow for TreeCompanionWindow {}
impl TWindow for TreeCompanionWindow {}
impl TEvtHandler for TreeCompanionWindow {}
impl TObject for TreeCompanionWindow { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeCompanionWindow {
    pub fn from(ptr: *mut c_void) -> TreeCompanionWindow { TreeCompanionWindow { ptr: ptr } }
    pub fn null() -> TreeCompanionWindow { TreeCompanionWindow::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTreeCompanionWindow](http://docs.wxwidgets.org/3.0/classwx_tree_companion_window.html) class.
pub trait TTreeCompanionWindow : TWindow {
}

/// Wraps the wxWidgets' [wxTreeLayout](http://docs.wxwidgets.org/3.0/classwx_tree_layout.html) class.
pub struct TreeLayout { ptr: *mut c_void }
impl TTreeLayout for TreeLayout {}
impl TObject for TreeLayout { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeLayout {
    pub fn from(ptr: *mut c_void) -> TreeLayout { TreeLayout { ptr: ptr } }
    pub fn null() -> TreeLayout { TreeLayout::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTreeLayout](http://docs.wxwidgets.org/3.0/classwx_tree_layout.html) class.
pub trait TTreeLayout : TObject {
}

/// Wraps the wxWidgets' [wxTreeLayoutStored](http://docs.wxwidgets.org/3.0/classwx_tree_layout_stored.html) class.
pub struct TreeLayoutStored { ptr: *mut c_void }
impl TTreeLayoutStored for TreeLayoutStored {}
impl TTreeLayout for TreeLayoutStored {}
impl TObject for TreeLayoutStored { fn ptr(&self) -> *mut c_void { self.ptr } }

impl TreeLayoutStored {
    pub fn from(ptr: *mut c_void) -> TreeLayoutStored { TreeLayoutStored { ptr: ptr } }
    pub fn null() -> TreeLayoutStored { TreeLayoutStored::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxTreeLayoutStored](http://docs.wxwidgets.org/3.0/classwx_tree_layout_stored.html) class.
pub trait TTreeLayoutStored : TTreeLayout {
}

/// Wraps the wxWidgets' [wxGauge95](http://docs.wxwidgets.org/3.0/classwx_gauge_95.html) class.
pub struct Gauge95 { ptr: *mut c_void }
impl TGauge95 for Gauge95 {}
impl TGauge for Gauge95 {}
impl TControl for Gauge95 {}
impl TWindow for Gauge95 {}
impl TEvtHandler for Gauge95 {}
impl TObject for Gauge95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Gauge95 {
    pub fn from(ptr: *mut c_void) -> Gauge95 { Gauge95 { ptr: ptr } }
    pub fn null() -> Gauge95 { Gauge95::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxGauge95](http://docs.wxwidgets.org/3.0/classwx_gauge_95.html) class.
pub trait TGauge95 : TGauge {
}

/// Wraps the wxWidgets' [wxGaugeMSW](http://docs.wxwidgets.org/3.0/classwx_gauge_msw.html) class.
pub struct GaugeMSW { ptr: *mut c_void }
impl TGaugeMSW for GaugeMSW {}
impl TGauge for GaugeMSW {}
impl TControl for GaugeMSW {}
impl TWindow for GaugeMSW {}
impl TEvtHandler for GaugeMSW {}
impl TObject for GaugeMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl GaugeMSW {
    pub fn from(ptr: *mut c_void) -> GaugeMSW { GaugeMSW { ptr: ptr } }
    pub fn null() -> GaugeMSW { GaugeMSW::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxGaugeMSW](http://docs.wxwidgets.org/3.0/classwx_gauge_msw.html) class.
pub trait TGaugeMSW : TGauge {
}

/// Wraps the wxWidgets' [wxSlider95](http://docs.wxwidgets.org/3.0/classwx_slider_95.html) class.
pub struct Slider95 { ptr: *mut c_void }
impl TSlider95 for Slider95 {}
impl TSlider for Slider95 {}
impl TControl for Slider95 {}
impl TWindow for Slider95 {}
impl TEvtHandler for Slider95 {}
impl TObject for Slider95 { fn ptr(&self) -> *mut c_void { self.ptr } }

impl Slider95 {
    pub fn from(ptr: *mut c_void) -> Slider95 { Slider95 { ptr: ptr } }
    pub fn null() -> Slider95 { Slider95::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSlider95](http://docs.wxwidgets.org/3.0/classwx_slider_95.html) class.
pub trait TSlider95 : TSlider {
}

/// Wraps the wxWidgets' [wxSliderMSW](http://docs.wxwidgets.org/3.0/classwx_slider_msw.html) class.
pub struct SliderMSW { ptr: *mut c_void }
impl TSliderMSW for SliderMSW {}
impl TSlider for SliderMSW {}
impl TControl for SliderMSW {}
impl TWindow for SliderMSW {}
impl TEvtHandler for SliderMSW {}
impl TObject for SliderMSW { fn ptr(&self) -> *mut c_void { self.ptr } }

impl SliderMSW {
    pub fn from(ptr: *mut c_void) -> SliderMSW { SliderMSW { ptr: ptr } }
    pub fn null() -> SliderMSW { SliderMSW::from(0 as *mut c_void) }
    
}

/// Methods of the wxWidgets' [wxSliderMSW](http://docs.wxwidgets.org/3.0/classwx_slider_msw.html) class.
pub trait TSliderMSW : TSlider {
}

