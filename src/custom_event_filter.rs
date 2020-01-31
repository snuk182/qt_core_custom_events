use cpp_core::*;
/// C++ type: <span style='color: green;'>```QCustomEventFilter```</span>
#[repr(C)]
pub struct CustomEventFilter(u8);

impl CustomEventFilter {
  pub fn new<F>(filter: F) -> CppBox<::custom_event_filter::CustomEventFilter>
      where F: for<'a,'b> FnMut(&'a mut ::qt_core::QObject,&'b mut ::qt_core::QEvent) -> bool {
    let boxed_filter: Box<Box<dyn FnMut(&mut ::qt_core::QObject,&mut ::qt_core::QEvent) -> bool>> = Box::new(Box::new(filter));
    let filter_ptr = Box::into_raw(boxed_filter) as *mut _ as *mut ::libc::c_void;

    let ffi_result = unsafe { ::ffi::qt_core_c_QCustomEventFilter_new(Some(handler), filter_ptr) };
    unsafe { CppBox::new(MutPtr::from_raw(ffi_result)).unwrap() }
  }
  pub fn clear(&mut self) {
    unsafe {
      let filter_ptr = ::ffi::qt_core_c_QCustomEventFilter_clear(self as *mut ::custom_event_filter::CustomEventFilter) as *mut Box<dyn FnMut(&mut ::qt_core::QObject,&mut ::qt_core::QEvent) -> bool>;
      if !filter_ptr.is_null() {
        let _:Box<Box<dyn FnMut(&mut ::qt_core::QObject,&mut ::qt_core::QEvent) -> bool>>  = Box::from_raw(filter_ptr);
      }
    }
  }
}
impl Drop for ::custom_event_filter::CustomEventFilter {
  fn drop(&mut self) {
  	self.clear();
    unsafe {
      ::ffi::qt_core_c_QCustomEventFilter_delete(self as *mut ::custom_event_filter::CustomEventFilter)
    }
  }
}
impl CppDeletable for ::custom_event_filter::CustomEventFilter {
  unsafe fn delete(&mut self){
      self.clear();
      ::ffi::qt_core_c_QCustomEventFilter_delete(self as *mut ::custom_event_filter::CustomEventFilter)
  }
}
impl StaticUpcast<::qt_core::QObject> for ::custom_event_filter::CustomEventFilter {
  unsafe fn static_upcast_mut(ptr: MutPtr<Self>) -> MutPtr<::qt_core::QObject> {
    let ffi_result = ::ffi::qt_core_c_QCustomEventFilter_G_static_cast_QObject_ptr(ptr.as_mut_raw_ptr() as *mut ::custom_event_filter::CustomEventFilter);
    MutPtr::from_raw(ffi_result)
  }

  unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<::qt_core::QObject> {
    let ffi_result = ::ffi::qt_core_c_QCustomEventFilter_G_static_cast_QObject_ptr(ptr.as_raw_ptr() as *const ::custom_event_filter::CustomEventFilter as *mut ::custom_event_filter::CustomEventFilter);
    Ptr::from_raw(ffi_result)
  }
}
extern "C" fn handler(filter_ptr: *mut ::libc::c_void, target: *mut ::qt_core::QObject, event: *mut ::qt_core::QEvent) -> bool {
    let filter: &mut Box<dyn FnMut(&mut ::qt_core::QObject,&mut ::qt_core::QEvent) -> bool> = unsafe { ::std::mem::transmute(filter_ptr) };
    unsafe { filter(&mut *target, &mut *event) as bool }
}