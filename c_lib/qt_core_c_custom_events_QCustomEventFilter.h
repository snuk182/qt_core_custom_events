#ifndef QT_CORE_C_QCUSTOMEVENTFILTER_H
#define QT_CORE_C_QCUSTOMEVENTFILTER_H

#include "qt_core_c_custom_events_global.h"

class QCustomEventFilter : public QObject {
  //Q_OBJECT
private:
  void *data;
  bool (*customFilter)(void*,QObject*,QEvent*);
public:
  QCustomEventFilter(QObject *parent = 0): QObject(parent),data(0),customFilter(0) {};
  bool eventFilter(QObject* object,QEvent* event) override {
    if (this->customFilter != 0) {
      return this->customFilter(this->data, object, event);
    } else {
      return QObject::eventFilter(object, event);
    }
  }
  void set(bool (*customFilter)(void*,QObject*,QEvent*), void* data) {
    this->customFilter = customFilter;
    this->data = data;
  }
  void* clear() {
    void* ptr = this->data;
    this->data = 0;
    this->customFilter = 0;
    return ptr;
  }
};

extern "C" {
QT_CORE_C_CUSTOM_EVENTS_EXPORT QCustomEventFilter* qt_core_c_QCustomEventFilter_new(bool (*customFilter)(void*,QObject*,QEvent*),void *data);
QT_CORE_C_CUSTOM_EVENTS_EXPORT void* qt_core_c_QCustomEventFilter_clear(QCustomEventFilter* this_ptr);
QT_CORE_C_CUSTOM_EVENTS_EXPORT void qt_core_c_QCustomEventFilter_delete(QCustomEventFilter* this_ptr);
QT_CORE_C_CUSTOM_EVENTS_EXPORT QObject* qt_core_c_QCustomEventFilter_G_static_cast_QObject_ptr(QCustomEventFilter* ptr);
QT_CORE_C_CUSTOM_EVENTS_EXPORT QCustomEventFilter* qt_core_c_QCustomEventFilter_G_static_cast_QCustomEventFilter_ptr_QObject(QObject* ptr);
} // extern "C"

#endif // QT_CORE_C_QCUSTOMEVENTFILTER_H
