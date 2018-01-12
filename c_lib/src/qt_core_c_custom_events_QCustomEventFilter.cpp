#include "qt_core_c_custom_events_QCustomEventFilter.h"

QCustomEventFilter* qt_core_c_QCustomEventFilter_new(bool (*customFilter)(void*,QObject*,QEvent*),void* data) {
  QCustomEventFilter* ef = new QCustomEventFilter();
  ef->set(customFilter,data);
  return ef;
}
void* qt_core_c_QCustomEventFilter_clear(QCustomEventFilter* this_ptr) {
  return this_ptr->clear();
}
void qt_core_c_QCustomEventFilter_delete(QCustomEventFilter* this_ptr) {
  delete this_ptr;
}
QObject* qt_core_c_QCustomEventFilter_G_static_cast_QObject_ptr(QCustomEventFilter* ptr) {
  return static_cast<QObject*>(ptr);
}
QCustomEventFilter* qt_core_c_QCustomEventFilter_G_static_cast_QCustomEventFilter_ptr_QObject(QObject* ptr) {
  return static_cast<QCustomEventFilter*>(ptr);
}