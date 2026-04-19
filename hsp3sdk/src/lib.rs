#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod hsp3plugin;
pub mod hsp3struct;
pub mod hspvar_core;
pub mod hspwnd;

pub use hsp3plugin::*;
pub use hsp3struct::*;
pub use hspvar_core::*;
pub use hspwnd::*;

pub use windows_sys::Win32::Foundation::{COLORREF, HINSTANCE, HWND};
pub use windows_sys::Win32::Graphics::Gdi::{
    BitBlt, GetDC, RealizePalette, ReleaseDC, SelectPalette, BITMAPINFOHEADER, HBITMAP, HBRUSH,
    HDC, HFONT, HPALETTE, HPEN, LOGFONTA, RGBQUAD, SRCCOPY,
};

pub type BOOL = i32;
pub type BYTE = u8;
pub type DWORD = u32;
pub type HANDLE = isize;
pub type TCHAR = u16;
pub type PTRIVERTEX = *mut ::core::ffi::c_void;

#[repr(C)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}

#[repr(C)]
pub struct LPDRAWITEMSTRUCT {
    pub _unused: [u8; 0],
}

pub type HSPERROR = i32;
pub const HSPERROR_HSPERR_UNSUPPORTED_FUNCTION: HSPERROR = 1;
pub const HSPERROR_HSPERR_INVALID_FUNCPARAM: HSPERROR = 2;
pub const HSPERROR_HSPERR_DIVIDED_BY_ZERO: HSPERROR = 3;
pub const HSPERROR_HSPERR_TYPE_MISMATCH: HSPERROR = 4;
pub const HSPERROR_HSPERR_FILE_IO: HSPERROR = 5;
pub const HSPERROR_HSPERR_WRONG_EXPRESSION: HSPERROR = 6;
pub const HSPERROR_HSPERR_ARRAY_OVERFLOW: HSPERROR = 7;
pub const HSPERROR_HSPERR_ILLEGAL_FUNCTION: HSPERROR = 8;
