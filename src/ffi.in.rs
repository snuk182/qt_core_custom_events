extern "C" {
  pub fn qt_core_c_QCustomEventFilter_new(customFilter: Option<extern "C" fn(*mut ::libc::c_void,*mut ::qt_core::QObject,*mut ::qt_core::QEvent) -> bool>,filter: *mut ::libc::c_void) -> *mut ::custom_event_filter::CustomEventFilter;
  pub fn qt_core_c_QCustomEventFilter_clear(this_ptr: *mut ::custom_event_filter::CustomEventFilter) -> *mut ::libc::c_void;
  pub fn qt_core_c_QCustomEventFilter_delete(this_ptr: *mut ::custom_event_filter::CustomEventFilter);
  pub fn qt_core_c_QCustomEventFilter_G_static_cast_QObject_ptr(ptr: *mut ::custom_event_filter::CustomEventFilter) -> *mut ::qt_core::QObject;
  pub fn qt_core_c_QCustomEventFilter_G_static_cast_QCustomEventFilter_ptr_QObject(ptr: *mut ::qt_core::QObject) -> *mut ::custom_event_filter::CustomEventFilter;
}
