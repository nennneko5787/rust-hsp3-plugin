#![allow(non_camel_case_types, non_snake_case, dead_code)]

use super::hsp3struct::*;
use super::hspvar_core::*;
use crate::*;

pub const HSPOBJ_OPTION_SETFONT: i32 = 0x100;
pub const MESSAGE_HSPOBJ: i32 = 0x4000;
pub const HSPOBJ_LIMIT_DEFAULT: i32 = 1024;

pub const HSPOBJ_INPUT_STR: i32 = 2;
pub const HSPOBJ_INPUT_DOUBLE: i32 = 3;
pub const HSPOBJ_INPUT_INT: i32 = 4;
pub const HSPOBJ_INPUT_MULTILINE: i32 = 0x100;
pub const HSPOBJ_INPUT_READONLY: i32 = 0x200;
pub const HSPOBJ_INPUT_HSCROLL: i32 = 0x400;
pub const HSPOBJ_INPUT_NOWRAP: i32 = 0x800;
pub const HSPOBJ_INPUT_BACKCOLOR: i32 = 0x1000;

pub const HSPOBJ_NONE: i32 = 0;
pub const HSPOBJ_TAB_ENABLE: i32 = 1;
pub const HSPOBJ_TAB_DISABLE: i32 = 2;
pub const HSPOBJ_TAB_SKIP: i32 = 3;
pub const HSPOBJ_TAB_SELALLTEXT: i32 = 4;

pub const HSPOBJ_OPTION_LAYEROBJ: i32 = 0x8000;
pub const HSPOBJ_OPTION_LAYER_MIN: i32 = 0;
pub const HSPOBJ_OPTION_LAYER_BG: i32 = 1;
pub const HSPOBJ_OPTION_LAYER_NORMAL: i32 = 2;
pub const HSPOBJ_OPTION_LAYER_POSTEFF: i32 = 3;
pub const HSPOBJ_OPTION_LAYER_MAX: i32 = 4;
pub const HSPOBJ_OPTION_LAYER_MULTI: i32 = 0x100;

pub const HSPOBJ_LAYER_CMD_NONE: i32 = 0;
pub const HSPOBJ_LAYER_CMD_INIT: i32 = 1;
pub const HSPOBJ_LAYER_CMD_TERM: i32 = 2;
pub const HSPOBJ_LAYER_CMD_PRMI: i32 = 3;
pub const HSPOBJ_LAYER_CMD_PRMS: i32 = 4;
pub const HSPOBJ_LAYER_CMD_PRMD: i32 = 5;
pub const HSPOBJ_LAYER_CMD_DRAW: i32 = 6;
pub const HSPOBJ_LAYER_CMD_TIME: i32 = 7;

pub const HSPMES_FONT_EFFSIZE_DEFAULT: i32 = 1;
pub const HSPMES_NOCR: i32 = 1;
pub const HSPMES_SHADOW: i32 = 2;
pub const HSPMES_OUTLINE: i32 = 4;
pub const HSPMES_LIGHT: i32 = 8;
pub const HSPMES_GMODE: i32 = 16;

#[repr(C)]
pub struct HSP3VARSET {
    pub type_: i32,
    pub pval: *mut PVal,
    pub aptr: APTR,
    pub ptr: *mut ::std::ffi::c_void,
}

#[repr(C)]
pub struct HSP3BTNSET {
    pub normal_x: i16,
    pub normal_y: i16,
    pub push_x: i16,
    pub push_y: i16,
    pub focus_x: i16,
    pub focus_y: i16,
    pub ptr: *mut ::std::ffi::c_void,
}

#[repr(C)]
pub struct HSPOBJINFO {
    pub owmode: i16,
    pub option: i16,
    pub bm: *mut ::std::ffi::c_void,
    pub hCld: HWND,
    pub owid: i32,
    pub owsize: i32,
    pub varset: HSP3VARSET,
    pub func_notice: Option<unsafe extern "C" fn(*mut HSPOBJINFO, i32)>,
    pub func_objprm: Option<unsafe extern "C" fn(*mut HSPOBJINFO, i32, *mut ::std::ffi::c_void)>,
    pub func_delete: Option<unsafe extern "C" fn(*mut HSPOBJINFO)>,
    pub br_back: HBRUSH,
    pub color_back: COLORREF,
    pub color_text: COLORREF,
    pub exinfo1: i32,
    pub exinfo2: i32,
    pub hspctx: *mut HSPCTX,
}

pub const BMSCR_FLAG_NOUSE: i32 = 0;
pub const BMSCR_FLAG_INUSE: i32 = 1;
pub const BMSCR_PALMODE_FULLCOLOR: i32 = 0;
pub const BMSCR_PALMODE_PALETTECOLOR: i32 = 1;

pub const HSPWND_TYPE_NONE: i32 = 0;
pub const HSPWND_TYPE_BUFFER: i32 = 1;
pub const HSPWND_TYPE_MAIN: i32 = 2;
pub const HSPWND_TYPE_BGSCR: i32 = 3;
pub const HSPWND_TYPE_SSPREVIEW: i32 = 4;

pub const BMSCR_SAVEPOS_MOSUEX: usize = 0;
pub const BMSCR_SAVEPOS_MOSUEY: usize = 1;
pub const BMSCR_SAVEPOS_MOSUEZ: usize = 2;
pub const BMSCR_SAVEPOS_MOSUEW: usize = 3;
pub const BMSCR_SAVEPOS_MAX: usize = 4;

pub type MM_NOTIFY_FUNC = Option<unsafe extern "C" fn(*mut ::std::ffi::c_void)>;

#[repr(C)]
pub struct BMSCR {
    pub flag: i32,
    pub sx: i32,
    pub sy: i32,
    pub palmode: i32,
    pub hdc: HDC,
    pub pBit: *mut BYTE,
    pub pbi: *mut BITMAPINFOHEADER,
    pub dib: HBITMAP,
    pub old: HBITMAP,
    pub pal: *mut RGBQUAD,
    pub hpal: HPALETTE,
    pub holdpal: HPALETTE,
    pub pals: i32,
    pub hwnd: HWND,
    pub hInst: HINSTANCE,
    pub infsize: i32,
    pub bmpsize: i32,
    pub type_: i32,
    pub wid: i32,
    pub fl_dispw: i16,
    pub fl_udraw: i16,
    pub wx: i32,
    pub wy: i32,
    pub wchg: i32,
    pub viewx: i32,
    pub viewy: i32,
    pub lx: i32,
    pub ly: i32,
    pub cx: i32,
    pub cy: i32,
    pub ox: i32,
    pub oy: i32,
    pub py: i32,
    pub texty: i32,
    pub gx: i32,
    pub gy: i32,
    pub gmode: i32,
    pub hbr: HBRUSH,
    pub hpn: HPEN,
    pub hfont: HFONT,
    pub holdfon: HFONT,
    pub color: COLORREF,
    pub textspeed: i32,
    pub cx2: i32,
    pub cy2: i32,
    pub tex: i32,
    pub tey: i32,
    pub prtmes: *mut i8,
    pub focflg: i32,
    pub objmode: i32,
    pub logfont: *mut LOGFONTA,
    pub logopt: [i32; 14],
    pub style: i32,
    pub gfrate: i32,
    pub tabmove: i32,
    pub sx2: i32,
    pub printsize: SIZE,
    pub objstyle: i32,
    pub mem_obj: *mut HSPOBJINFO,
    pub objmax: i32,
    pub objlimit: i32,
    pub savepos: [i16; BMSCR_SAVEPOS_MAX],
    pub master_hspwnd: *mut ::std::ffi::c_void,
    pub palcolor: i16,
    pub textstyle: i16,
    pub framesx: i16,
    pub framesy: i16,
    pub imgbtn: i32,
    pub btn_x1: i16,
    pub btn_y1: i16,
    pub btn_x2: i16,
    pub btn_y2: i16,
    pub btn_x3: i16,
    pub btn_y3: i16,
    pub divx: i16,
    pub divy: i16,
    pub divsx: i16,
    pub divsy: i16,
    pub celofsx: i16,
    pub celofsy: i16,
    pub objcolor: COLORREF,
    pub fonteff_size: i32,
    pub vp_flag: i32,
    pub vp_matrix: [f32; 16],
}

#[repr(C)]
pub struct Bmscr {
    pub _unused: [u8; 0],
}

#[repr(C)]
pub struct HspWnd {
    pub _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn SetObjectEventNoticePtr(ptr: *mut i32);
}
