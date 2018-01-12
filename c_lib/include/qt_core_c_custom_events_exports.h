#ifndef QT_CORE_C_CUSTOM_EVENTS_EXPORTS_H
#define QT_CORE_C_CUSTOM_EVENTS_EXPORTS_H

// This header creates a definition required to export the library's
// symbols properly on all platforms.

#ifdef _WIN32
    #ifdef QT_CORE_C_LIBRARY
        #define QT_CORE_C_CUSTOM_EVENTS_EXPORT __declspec(dllexport)
    #else
        #define QT_CORE_C_CUSTOM_EVENTS_EXPORT __declspec(dllimport)
    #endif
#else
    #define QT_CORE_C_CUSTOM_EVENTS_EXPORT
#endif

#endif // QT_CORE_C_CUSTOM_EVENTS_EXPORTS_H

