/// C++ type: <span style='color: green;'>```QCustomEventFilter```</span>
#[repr(C)]
pub struct CustomEventFilter(u8);

impl CustomEventFilter {
  pub fn new<F>(filter: F) -> ::qt_core::cpp_utils::CppBox<::custom_event_filter::CustomEventFilter>
      where F: for<'a,'b> FnMut(&'a mut ::qt_core::object::Object,&'b ::qt_core::event::Event) -> bool {
    let boxed_filter: Box<Box<FnMut(&mut ::qt_core::object::Object,&::qt_core::event::Event) -> bool>> = Box::new(Box::new(filter));
    let filter_ptr = Box::into_raw(boxed_filter) as *mut _ as *mut ::libc::c_void;

    let ffi_result = unsafe { ::ffi::qt_core_c_QCustomEventFilter_new(Some(handler), filter_ptr) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
impl Drop for ::custom_event_filter::CustomEventFilter {
  fn drop(&mut self) {
    unsafe {
      let filter_ptr = ::ffi::qt_core_c_QCustomEventFilter_clear(self as *mut ::custom_event_filter::CustomEventFilter) as *mut Box<FnMut(&mut ::qt_core::object::Object,&::qt_core::event::Event) -> bool>;
      if !filter_ptr.is_null() {
        let _:Box<Box<FnMut(&mut ::qt_core::object::Object,&::qt_core::event::Event) -> bool>>  = Box::from_raw(filter_ptr);
      }
      ::ffi::qt_core_c_QCustomEventFilter_delete(self as *mut ::custom_event_filter::CustomEventFilter)
    }
  }
}
// HERE THE CLOSURE WILL LEAK IF DROP NOT CALLED BEFORE!!!
impl ::cpp_utils::CppDeletable for ::custom_event_filter::CustomEventFilter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QCustomEventFilter_delete
  }
}
impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::custom_event_filter::CustomEventFilter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QCustomEventFilter_G_static_cast_QObject_ptr(self as *mut ::custom_event_filter::CustomEventFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QCustomEventFilter_G_static_cast_QObject_ptr(self as *const ::custom_event_filter::CustomEventFilter as *mut ::custom_event_filter::CustomEventFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}
extern "C" fn handler(filter_ptr: *mut ::libc::c_void, target: *mut ::qt_core::object::Object, event: *const ::qt_core::event::Event) -> bool {
    let filter: &mut Box<FnMut(&mut ::qt_core::object::Object,&::qt_core::event::Event) -> bool> = unsafe { ::std::mem::transmute(filter_ptr) };
    unsafe { filter(&mut *target, &*event) as bool }
}