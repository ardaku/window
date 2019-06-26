use std::ffi::c_void;

#[cfg(target_platform = "windows")]
mod native {
    //    typedef HDC     EGLNativeDisplayType;
    //    typedef HBITMAP EGLNativePixmapType;
    //    typedef HWND    EGLNativeWindowType;
}

#[cfg(target_arch = "wasm")]
mod native {
    pub type NativeDisplayType = i32;
    pub type EGLNativePixmapType = i32;
    pub type EGLNativeWindowType = i32;
}

// #[cfg(target_arch = "wasm")]

/*#[cfg(target_os = "symbian"]
mod native {
    typedef int   EGLNativeDisplayType;
    typedef void *EGLNativePixmapType;
    typedef void *EGLNativeWindowType;
}

#elif defined(WL_EGL_PLATFORM)*/

#[cfg(target_os = "linux")]
pub type NativeDisplayType = *mut c_void;
//    wl_egl_pixmap  *EGLNativePixmapType;
//    wl_egl_window  *EGLNativeWindowType;

/* X11 (tentative)  */
/*#include <X11/Xlib.h>
#include <X11/Xutil.h>

typedef Display *EGLNativeDisplayType;
typedef Pixmap   EGLNativePixmapType;
typedef Window   EGLNativeWindowType;

#elif defined(__GBM__)

typedef struct gbm_device  *EGLNativeDisplayType;
typedef struct gbm_bo      *EGLNativePixmapType;
typedef void               *EGLNativeWindowType;
*/

#[cfg(target_os = "android")]
mod native {
    //    struct ANativeWindow;
    //    struct egl_native_pixmap_t;

    pub type NativeDisplayType = *mut c_void;
    //    typedef struct egl_native_pixmap_t*     EGLNativePixmapType;
    //    typedef struct ANativeWindow*           EGLNativeWindowType;
}

/*#elif defined(USE_OZONE)

typedef intptr_t EGLNativeDisplayType;
typedef intptr_t EGLNativePixmapType;
typedef intptr_t EGLNativeWindowType;

#elif defined(__APPLE__)

typedef int   EGLNativeDisplayType;
typedef void *EGLNativePixmapType;
typedef void *EGLNativeWindowType;

#elif defined(__HAIKU__)

#include <kernel/image.h>

typedef void              *EGLNativeDisplayType;
typedef khronos_uintptr_t  EGLNativePixmapType;
typedef khronos_uintptr_t  EGLNativeWindowType;

#else
#error "Platform not recognized"
#endif

/* EGL 1.2 types, renamed for consistency in EGL 1.3 */
typedef EGLNativeDisplayType NativeDisplayType;
typedef EGLNativePixmapType  NativePixmapType;
typedef EGLNativeWindowType  NativeWindowType;*/

// pub use self::native::*;
