#![allow(non_camel_case_types, non_snake_case, dead_code)]

use super::hspvar_core::*;
use crate::*;

pub type HSPREAL = f64;
pub type HSPLPTR = i32;

pub const TYPE_MARK: i32 = 0;
pub const TYPE_VAR: i32 = 1;
pub const TYPE_STRING: i32 = 2;
pub const TYPE_DNUM: i32 = 3;
pub const TYPE_INUM: i32 = 4;
pub const TYPE_STRUCT: i32 = 5;
pub const TYPE_XLABEL: i32 = 6;
pub const TYPE_LABEL: i32 = 7;
pub const TYPE_INTCMD: i32 = 8;
pub const TYPE_EXTCMD: i32 = 9;
pub const TYPE_EXTSYSVAR: i32 = 10;
pub const TYPE_CMPCMD: i32 = 11;
pub const TYPE_MODCMD: i32 = 12;
pub const TYPE_INTFUNC: i32 = 13;
pub const TYPE_SYSVAR: i32 = 14;
pub const TYPE_PROGCMD: i32 = 15;
pub const TYPE_DLLFUNC: i32 = 16;
pub const TYPE_DLLCTRL: i32 = 17;
pub const TYPE_USERDEF: i32 = 18;
pub const TYPE_ERROR: i32 = -1;
pub const TYPE_CALCERROR: i32 = -2;

pub const PARAM_OK: i32 = 0;
pub const PARAM_SPLIT: i32 = -1;
pub const PARAM_END: i32 = -2;
pub const PARAM_DEFAULT: i32 = -3;
pub const PARAM_ENDSPLIT: i32 = -4;

pub const HSP3_FUNC_MAX: i32 = 18;
pub const HSP3_TYPE_USER: i32 = 18;

pub const EXFLG_0: i32 = 0x1000;
pub const EXFLG_1: i32 = 0x2000;
pub const EXFLG_2: i32 = 0x4000;
pub const EXFLG_3: i32 = 0x8000;
pub const CSTYPE: i32 = 0x0fff;

#[repr(C)]
pub struct HSPHED {
    pub h1: i8,
    pub h2: i8,
    pub h3: i8,
    pub h4: i8,
    pub version: i32,
    pub max_val: i32,
    pub allsize: i32,
    pub pt_cs: i32,
    pub max_cs: i32,
    pub pt_ds: i32,
    pub max_ds: i32,
    pub pt_ot: i32,
    pub max_ot: i32,
    pub pt_dinfo: i32,
    pub max_dinfo: i32,
    pub pt_linfo: i32,
    pub max_linfo: i32,
    pub pt_finfo: i32,
    pub max_finfo: i32,
    pub pt_minfo: i32,
    pub max_minfo: i32,
    pub pt_finfo2: i32,
    pub max_finfo2: i32,
    pub pt_hpidat: i32,
    pub max_hpi: i16,
    pub max_varhpi: i16,
    pub bootoption: i32,
    pub runtime: i32,
    pub pt_sr: i32,
    pub max_sr: i32,
    pub pt_exopt: i32,
    pub max_exopt: i32,
}

pub const HSPHED_BOOTOPT_DEBUGWIN: i32 = 1;
pub const HSPHED_BOOTOPT_WINHIDE: i32 = 2;
pub const HSPHED_BOOTOPT_DIRSAVE: i32 = 4;
pub const HSPHED_BOOTOPT_SAVER: i32 = 0x100;
pub const HSPHED_BOOTOPT_RUNTIME: i32 = 0x1000;
pub const HSPHED_BOOTOPT_NOMMTIMER: i32 = 0x2000;
pub const HSPHED_BOOTOPT_NOGDIP: i32 = 0x4000;
pub const HSPHED_BOOTOPT_FLOAT32: i32 = 0x8000;
pub const HSPHED_BOOTOPT_ORGRND: i32 = 0x10000;
pub const HSPHED_BOOTOPT_UTF8: i32 = 0x20000;
pub const HSPHED_BOOTOPT_HSP64: i32 = 0x40000;
pub const HSPHED_BOOTOPT_IORESUME: i32 = 0x80000;

pub const HPIDAT_FLAG_TYPEFUNC: i32 = 0;
pub const HPIDAT_FLAG_SELFFUNC: i32 = -1;
pub const HPIDAT_FLAG_VARFUNC: i32 = 1;
pub const HPIDAT_FLAG_DLLFUNC: i32 = 2;

pub const HSPHED_EXOPTION_TAG_NONE: i32 = 0;
pub const HSPHED_EXOPTION_TAG_DSINDEX: i32 = 1;
pub const HSPHED_EXOPTION_TAG_SIZEX: i32 = 2;
pub const HSPHED_EXOPTION_TAG_SIZEY: i32 = 3;
pub const HSPHED_EXOPTION_TAG_SYSREQ: i32 = 4;

#[repr(C)]
pub struct MEM_HPIDAT {
    pub flag: i16,
    pub option: i16,
    pub libname: i32,
    pub funcname: i32,
    pub libptr: *mut ::std::ffi::c_void,
}

#[repr(C)]
pub struct HPIDAT {
    pub flag: i16,
    pub option: i16,
    pub libname: i32,
    pub funcname: i32,
    pub p_libptr: i32,
}

pub const LIBDAT_FLAG_NONE: i32 = 0;
pub const LIBDAT_FLAG_DLL: i32 = 1;
pub const LIBDAT_FLAG_DLLINIT: i32 = 2;
pub const LIBDAT_FLAG_MODULE: i32 = 3;
pub const LIBDAT_FLAG_COMOBJ: i32 = 4;

#[repr(C)]
pub struct LIBDAT {
    pub flag: i32,
    pub nameidx: i32,
    pub hlib: *mut ::std::ffi::c_void,
    pub clsid: i32,
}

#[repr(C)]
pub struct HED_LIBDAT {
    pub flag: i32,
    pub nameidx: i32,
    pub p_hlib: i32,
    pub clsid: i32,
}

pub const MPTYPE_NONE: i32 = 0;
pub const MPTYPE_VAR: i32 = 1;
pub const MPTYPE_STRING: i32 = 2;
pub const MPTYPE_DNUM: i32 = 3;
pub const MPTYPE_INUM: i32 = 4;
pub const MPTYPE_STRUCT: i32 = 5;
pub const MPTYPE_LABEL: i32 = 7;
pub const MPTYPE_LOCALVAR: i32 = -1;
pub const MPTYPE_ARRAYVAR: i32 = -2;
pub const MPTYPE_SINGLEVAR: i32 = -3;
pub const MPTYPE_FLOAT: i32 = -4;
pub const MPTYPE_STRUCTTAG: i32 = -5;
pub const MPTYPE_LOCALSTRING: i32 = -6;
pub const MPTYPE_MODULEVAR: i32 = -7;
pub const MPTYPE_PPVAL: i32 = -8;
pub const MPTYPE_PBMSCR: i32 = -9;
pub const MPTYPE_PVARPTR: i32 = -10;
pub const MPTYPE_IMODULEVAR: i32 = -11;
pub const MPTYPE_IOBJECTVAR: i32 = -12;
pub const MPTYPE_LOCALWSTR: i32 = -13;
pub const MPTYPE_FLEXSPTR: i32 = -14;
pub const MPTYPE_FLEXWPTR: i32 = -15;
pub const MPTYPE_PTR_REFSTR: i32 = -16;
pub const MPTYPE_PTR_EXINFO: i32 = -17;
pub const MPTYPE_PTR_DPMINFO: i32 = -18;
pub const MPTYPE_NULLPTR: i32 = -19;
pub const MPTYPE_TMODULEVAR: i32 = -20;

pub const STRUCTPRM_SUBID_STACK: i32 = -1;
pub const STRUCTPRM_SUBID_STID: i32 = -2;
pub const STRUCTPRM_SUBID_DLL: i32 = -3;
pub const STRUCTPRM_SUBID_DLLINIT: i32 = -4;
pub const STRUCTPRM_SUBID_OLDDLL: i32 = -5;
pub const STRUCTPRM_SUBID_OLDDLLINIT: i32 = -6;
pub const STRUCTPRM_SUBID_COMOBJ: i32 = -7;
pub const STRUCTCODE_THISMOD: i32 = -1;
pub const TYPE_OFFSET_COMOBJ: i32 = 0x1000;

#[repr(C)]
pub struct STRUCTPRM {
    pub mptype: i16,
    pub subid: i16,
    pub offset: i32,
}

pub const STRUCTDAT_OT_NONE: i32 = 0;
pub const STRUCTDAT_OT_CLEANUP: i32 = 1;
pub const STRUCTDAT_OT_STATEMENT: i32 = 2;
pub const STRUCTDAT_OT_FUNCTION: i32 = 4;
pub const STRUCTDAT_INDEX_FUNC: i32 = -1;
pub const STRUCTDAT_INDEX_CFUNC: i32 = -2;
pub const STRUCTDAT_INDEX_STRUCT: i32 = -3;
pub const STRUCTDAT_FUNCFLAG_CLEANUP: i32 = 0x10000;

#[repr(C)]
pub struct STRUCTDAT {
    pub index: i16,
    pub subid: i16,
    pub prmindex: i32,
    pub prmmax: i32,
    pub nameidx: i32,
    pub size: i32,
    pub otindex: i32,
    pub proc_: *mut ::std::ffi::c_void,
    pub funcflag: i32,
}

#[repr(C)]
pub struct HED_STRUCTDAT {
    pub index: i16,
    pub subid: i16,
    pub prmindex: i32,
    pub prmmax: i32,
    pub nameidx: i32,
    pub size: i32,
    pub otindex: i32,
    pub funcflag: i32,
}

#[repr(C)]
pub struct MPVarData {
    pub pval: *mut PVal,
    pub aptr: APTR,
}

#[repr(C)]
pub struct MPModVarData {
    pub subid: i16,
    pub magic: i16,
    pub pval: *mut PVal,
    pub aptr: APTR,
}

pub const MODVAR_MAGICCODE: u16 = 0x55aa;
pub const IRQ_FLAG_NONE: i16 = 0;
pub const IRQ_FLAG_DISABLE: i16 = 1;
pub const IRQ_FLAG_ENABLE: i16 = 2;
pub const IRQ_OPT_GOTO: i16 = 0;
pub const IRQ_OPT_GOSUB: i16 = 1;
pub const IRQ_OPT_CALLBACK: i16 = 2;

#[repr(C)]
pub struct IRQDAT {
    pub flag: i16,
    pub opt: i16,
    pub custom: i32,
    pub custom2: i32,
    pub iparam: i32,
    pub ptr: *mut u16,
    pub callback: Option<unsafe extern "C" fn(*mut IRQDAT, i32, i32)>,
}

pub type HSP3_CMDFUNC = Option<unsafe extern "C" fn(i32) -> i32>;
pub type HSP3_REFFUNC = Option<unsafe extern "C" fn(*mut i32, i32) -> *mut ::std::ffi::c_void>;
pub type HSP3_TERMFUNC = Option<unsafe extern "C" fn(i32) -> i32>;
pub type HSP3_MSGFUNC = Option<unsafe extern "C" fn(i32, i32, i32) -> i32>;
pub type HSP3_EVENTFUNC =
    Option<unsafe extern "C" fn(i32, i32, i32, *mut ::std::ffi::c_void) -> i32>;

#[repr(C)]
pub struct HSPEXINFO30 {
    pub ver: i16,
    pub min: i16,
    pub er: *mut i32,
    pub pstr: *mut i8,
    pub stmp: *mut i8,
    pub mpval: *mut *mut PVal,
    pub actscr: *mut i32,
    pub nptype: *mut i32,
    pub npval: *mut i32,
    pub strsize: *mut i32,
    pub refstr: *mut i8,
    pub HspFunc_prm_getv: Option<unsafe extern "C" fn() -> *mut ::std::ffi::c_void>,
    pub HspFunc_prm_geti: Option<unsafe extern "C" fn() -> i32>,
    pub HspFunc_prm_getdi: Option<unsafe extern "C" fn(i32) -> i32>,
    pub HspFunc_prm_gets: Option<unsafe extern "C" fn() -> *mut i8>,
    pub HspFunc_prm_getds: Option<unsafe extern "C" fn(*const i8) -> *mut i8>,
    pub HspFunc_val_realloc: Option<unsafe extern "C" fn(*mut PVal, i32, i32) -> i32>,
    pub HspFunc_fread:
        Option<unsafe extern "C" fn(*mut i8, *mut ::std::ffi::c_void, i32, i32) -> i32>,
    pub HspFunc_fsize: Option<unsafe extern "C" fn(*mut i8) -> i32>,
    pub HspFunc_getbmscr: Option<unsafe extern "C" fn(i32) -> *mut ::std::ffi::c_void>,
    pub HspFunc_getobj: Option<unsafe extern "C" fn(i32, i32, *mut ::std::ffi::c_void) -> i32>,
    pub HspFunc_setobj: Option<unsafe extern "C" fn(i32, i32, *const ::std::ffi::c_void) -> i32>,
    pub npexflg: *mut i32,
    pub hspctx: *mut HSPCTX,
    pub HspFunc_addobj: Option<unsafe extern "C" fn(i32) -> i32>,
    pub HspFunc_puterror: Option<unsafe extern "C" fn(HSPERROR)>,
    pub HspFunc_getproc: Option<unsafe extern "C" fn(i32) -> *mut HspVarProc>,
    pub HspFunc_seekproc: Option<unsafe extern "C" fn(*const i8) -> *mut HspVarProc>,
    pub HspFunc_prm_next: Option<unsafe extern "C" fn()>,
    pub HspFunc_prm_get: Option<unsafe extern "C" fn() -> i32>,
    pub HspFunc_prm_getd: Option<unsafe extern "C" fn() -> f64>,
    pub HspFunc_prm_getdd: Option<unsafe extern "C" fn(f64) -> f64>,
    pub HspFunc_prm_getlb: Option<unsafe extern "C" fn() -> *mut u16>,
    pub HspFunc_prm_getpval: Option<unsafe extern "C" fn() -> *mut PVal>,
    pub HspFunc_prm_getva: Option<unsafe extern "C" fn(*mut *mut PVal) -> APTR>,
    pub HspFunc_prm_setva:
        Option<unsafe extern "C" fn(*mut PVal, APTR, i32, *const ::std::ffi::c_void)>,
    pub HspFunc_malloc: Option<unsafe extern "C" fn(i32) -> *mut i8>,
    pub HspFunc_free: Option<unsafe extern "C" fn(*mut ::std::ffi::c_void)>,
    pub HspFunc_expand: Option<unsafe extern "C" fn(*mut i8, i32) -> *mut i8>,
    pub HspFunc_addirq: Option<unsafe extern "C" fn() -> *mut IRQDAT>,
    pub HspFunc_hspevent:
        Option<unsafe extern "C" fn(i32, i32, i32, *mut ::std::ffi::c_void) -> i32>,
    pub HspFunc_registvar: Option<unsafe extern "C" fn(i32, HSPVAR_COREFUNC)>,
    pub HspFunc_setpc: Option<unsafe extern "C" fn(*const u16)>,
    pub HspFunc_call: Option<unsafe extern "C" fn(*const u16)>,
    pub HspFunc_mref: Option<unsafe extern "C" fn(*mut PVal, i32)>,
    pub HspFunc_dim: Option<unsafe extern "C" fn(*mut PVal, i32, i32, i32, i32, i32, i32)>,
    pub HspFunc_redim: Option<unsafe extern "C" fn(*mut PVal, i32, i32)>,
    pub HspFunc_array: Option<unsafe extern "C" fn(*mut PVal, i32)>,
}

#[repr(C)]
pub struct HSPEXINFO {
    pub ver: i16,
    pub min: i16,
    pub er: *mut i32,
    pub pstr: *mut i8,
    pub stmp: *mut i8,
    pub mpval: *mut *mut PVal,
    pub actscr: *mut i32,
    pub nptype: *mut i32,
    pub npval: *mut i32,
    pub strsize: *mut i32,
    pub refstr: *mut i8,
    pub HspFunc_prm_getv: Option<unsafe extern "C" fn() -> *mut ::std::ffi::c_void>,
    pub HspFunc_prm_geti: Option<unsafe extern "C" fn() -> i32>,
    pub HspFunc_prm_getdi: Option<unsafe extern "C" fn(i32) -> i32>,
    pub HspFunc_prm_gets: Option<unsafe extern "C" fn() -> *mut i8>,
    pub HspFunc_prm_getds: Option<unsafe extern "C" fn(*const i8) -> *mut i8>,
    pub HspFunc_val_realloc: Option<unsafe extern "C" fn(*mut PVal, i32, i32) -> i32>,
    pub HspFunc_fread:
        Option<unsafe extern "C" fn(*mut i8, *mut ::std::ffi::c_void, i32, i32) -> i32>,
    pub HspFunc_fsize: Option<unsafe extern "C" fn(*mut i8) -> i32>,
    pub HspFunc_getbmscr: Option<unsafe extern "C" fn(i32) -> *mut ::std::ffi::c_void>,
    pub HspFunc_getobj: Option<unsafe extern "C" fn(i32, i32, *mut ::std::ffi::c_void) -> i32>,
    pub HspFunc_setobj: Option<unsafe extern "C" fn(i32, i32, *const ::std::ffi::c_void) -> i32>,
    pub npexflg: *mut i32,
    pub hspctx: *mut HSPCTX,
    pub HspFunc_addobj: Option<unsafe extern "C" fn(i32) -> i32>,
    pub HspFunc_puterror: Option<unsafe extern "C" fn(HSPERROR)>,
    pub HspFunc_getproc: Option<unsafe extern "C" fn(i32) -> *mut HspVarProc>,
    pub HspFunc_seekproc: Option<unsafe extern "C" fn(*const i8) -> *mut HspVarProc>,
    pub HspFunc_prm_next: Option<unsafe extern "C" fn()>,
    pub HspFunc_prm_get: Option<unsafe extern "C" fn() -> i32>,
    pub HspFunc_prm_getd: Option<unsafe extern "C" fn() -> f64>,
    pub HspFunc_prm_getdd: Option<unsafe extern "C" fn(f64) -> f64>,
    pub HspFunc_prm_getlb: Option<unsafe extern "C" fn() -> *mut u16>,
    pub HspFunc_prm_getpval: Option<unsafe extern "C" fn() -> *mut PVal>,
    pub HspFunc_prm_getva: Option<unsafe extern "C" fn(*mut *mut PVal) -> APTR>,
    pub HspFunc_prm_setva:
        Option<unsafe extern "C" fn(*mut PVal, APTR, i32, *const ::std::ffi::c_void)>,
    pub HspFunc_malloc: Option<unsafe extern "C" fn(i32) -> *mut i8>,
    pub HspFunc_free: Option<unsafe extern "C" fn(*mut ::std::ffi::c_void)>,
    pub HspFunc_expand: Option<unsafe extern "C" fn(*mut i8, i32) -> *mut i8>,
    pub HspFunc_addirq: Option<unsafe extern "C" fn() -> *mut IRQDAT>,
    pub HspFunc_hspevent:
        Option<unsafe extern "C" fn(i32, i32, i32, *mut ::std::ffi::c_void) -> i32>,
    pub HspFunc_registvar: Option<unsafe extern "C" fn(i32, HSPVAR_COREFUNC)>,
    pub HspFunc_setpc: Option<unsafe extern "C" fn(*const u16)>,
    pub HspFunc_call: Option<unsafe extern "C" fn(*const u16)>,
    pub HspFunc_mref: Option<unsafe extern "C" fn(*mut PVal, i32)>,
    pub HspFunc_dim: Option<unsafe extern "C" fn(*mut PVal, i32, i32, i32, i32, i32, i32)>,
    pub HspFunc_redim: Option<unsafe extern "C" fn(*mut PVal, i32, i32)>,
    pub HspFunc_array: Option<unsafe extern "C" fn(*mut PVal, i32)>,
    pub HspFunc_varname: Option<unsafe extern "C" fn(i32) -> *mut i8>,
    pub HspFunc_seekvar: Option<unsafe extern "C" fn(*const i8) -> i32>,
    pub HspFunc_prm_getns: Option<unsafe extern "C" fn() -> *mut i8>,
    pub HspFunc_prm_getnds: Option<unsafe extern "C" fn(*const i8) -> *mut i8>,
}

pub const HSP3_REPEAT_MAX: usize = 32;

#[repr(C)]
pub struct LOOPDAT {
    pub time: i32,
    pub cnt: i32,
    pub step: i32,
    pub pt: *mut u16,
}

pub const RUNMODE_RUN: i32 = 0;
pub const RUNMODE_WAIT: i32 = 1;
pub const RUNMODE_AWAIT: i32 = 2;
pub const RUNMODE_STOP: i32 = 3;
pub const RUNMODE_END: i32 = 4;
pub const RUNMODE_ERROR: i32 = 5;
pub const RUNMODE_RETURN: i32 = 6;
pub const RUNMODE_INTJUMP: i32 = 7;
pub const RUNMODE_ASSERT: i32 = 8;
pub const RUNMODE_LOGMES: i32 = 9;
pub const RUNMODE_EXITRUN: i32 = 10;
pub const RUNMODE_RESTART: i32 = 11;
pub const RUNMODE_MAX: i32 = 12;

#[repr(C)]
pub struct HSPCTX {
    pub hsphed: *mut HSPHED,
    pub mcs: *mut u16,
    pub mem_mcs: *mut u16,
    pub mem_mds: *mut i8,
    pub mem_di: *mut u8,
    pub mem_ot: *mut i32,
    pub mem_irq: *mut IRQDAT,
    pub irqmax: i32,
    pub iparam: i32,
    pub wparam: i32,
    pub lparam: i32,
    pub mem_var: *mut PVal,
    pub exinfo: HSPEXINFO30,
    pub runmode: i32,
    pub waitcount: i32,
    pub waitbase: i32,
    pub waittick: i32,
    pub lasttick: i32,
    pub sublev: i32,
    pub mem_loop: [LOOPDAT; HSP3_REPEAT_MAX],
    pub looplev: i32,
    pub err: HSPERROR,
    pub hspstat: i32,
    pub stat: i32,
    pub strsize: i32,
    pub refstr: *mut i8,
    pub fnbuffer: *mut i8,
    pub instance: *mut ::std::ffi::c_void,
    pub intwnd_id: i32,
    pub note_pval: *mut PVal,
    pub note_aptr: APTR,
    pub notep_pval: *mut PVal,
    pub notep_aptr: APTR,
    pub stmp: *mut i8,
    pub prmstack: *mut ::std::ffi::c_void,
    pub mem_linfo: *mut LIBDAT,
    pub mem_minfo: *mut STRUCTPRM,
    pub mem_finfo: *mut STRUCTDAT,
    pub retval_level: i32,
    pub endcode: i32,
    pub msgfunc: Option<unsafe extern "C" fn(*mut HSPCTX)>,
    pub wnd_parent: *mut ::std::ffi::c_void,
    pub refdval: f64,
    pub cmdline: *mut i8,
    pub exinfo2: *mut HSPEXINFO,
    pub prmstack_max: i32,
    pub dsindex: *mut i32,
    pub dsindex_size: i32,
    pub language: i32,
    pub callback_flag: i32,
    pub modfilename: *mut i8,
    pub tvfoldername: *mut i8,
    pub homefoldername: *mut i8,
    pub langcode: [i8; 4],
}

pub const HSPCTX_REFSTR_MAX: i32 = 4096;
pub const HSPCTX_CMDLINE_MAX: i32 = 256;
pub const HSPCTX_PATH_MAX: i32 = 64;

pub const HSPSTAT_NORMAL: i32 = 0;
pub const HSPSTAT_DEBUG: i32 = 1;
pub const HSPSTAT_SSAVER: i32 = 2;
pub const HSPSTAT_CONSOLE: i32 = 0x10;
pub const HSPSTAT_MAC: i32 = 0x80;
pub const HSPSTAT_DISH: i32 = 0x100;
pub const HSPSTAT_LINUX: i32 = 0x1000;
pub const HSPSTAT_UTF8: i32 = 0x20000;
pub const HSPSTAT_HSP64: i32 = 0x40000;

pub const TYPE_EX_SUBROUTINE: i32 = 0x100;
pub const TYPE_EX_CUSTOMFUNC: i32 = 0x101;
pub const TYPE_EX_ENDOFPARAM: i32 = 0x200;
pub const TYPE_EX_ARRAY_VARS: i32 = 0x201;
pub const TYPE_EX_LOCAL_VARS: i32 = 0x202;

pub const HSPCTX_LANGUAGE_EN: i32 = 0;
pub const HSPCTX_LANGUAGE_JP: i32 = 1;

#[repr(C)]
pub struct HSPROUTINE {
    pub stacklev: i32,
    pub mcsret: *mut u16,
    pub param: *mut STRUCTDAT,
    pub oldtack: *mut ::std::ffi::c_void,
    pub oldlev: i32,
}

pub const HSPEVENT_ENABLE_COMMAND: i32 = 1;
pub const HSPEVENT_ENABLE_HSPIRQ: i32 = 2;
pub const HSPEVENT_ENABLE_GETKEY: i32 = 4;
pub const HSPEVENT_ENABLE_FILE: i32 = 8;
pub const HSPEVENT_ENABLE_MEDIA: i32 = 16;
pub const HSPEVENT_ENABLE_PICLOAD: i32 = 32;

#[repr(C)]
pub struct HSP3TYPEINFO {
    pub type_: i16,
    pub option: i16,
    pub hspctx: *mut HSPCTX,
    pub hspexinfo: *mut HSPEXINFO,
    pub cmdfunc: HSP3_CMDFUNC,
    pub reffunc: HSP3_REFFUNC,
    pub termfunc: HSP3_TERMFUNC,
    pub msgfunc: HSP3_MSGFUNC,
    pub eventfunc: HSP3_EVENTFUNC,
}

pub const HSPIRQ_ONEXIT: i32 = 0;
pub const HSPIRQ_ONERROR: i32 = 1;
pub const HSPIRQ_ONKEY: i32 = 2;
pub const HSPIRQ_ONCLICK: i32 = 3;
pub const HSPIRQ_USERDEF: i32 = 4;
pub const HSPIRQ_MAX: i32 = 5;

pub const HSPEVENT_NONE: i32 = 0;
pub const HSPEVENT_COMMAND: i32 = 1;
pub const HSPEVENT_HSPIRQ: i32 = 2;
pub const HSPEVENT_GETKEY: i32 = 3;
pub const HSPEVENT_STICK: i32 = 4;
pub const HSPEVENT_FNAME: i32 = 5;
pub const HSPEVENT_FREAD: i32 = 6;
pub const HSPEVENT_FWRITE: i32 = 7;
pub const HSPEVENT_FEXIST: i32 = 8;
pub const HSPEVENT_FDELETE: i32 = 9;
pub const HSPEVENT_FMKDIR: i32 = 10;
pub const HSPEVENT_FCHDIR: i32 = 11;
pub const HSPEVENT_FCOPY: i32 = 12;
pub const HSPEVENT_FDIRLIST1: i32 = 13;
pub const HSPEVENT_FDIRLIST2: i32 = 14;
pub const HSPEVENT_GETPICSIZE: i32 = 15;
pub const HSPEVENT_PICLOAD: i32 = 16;
pub const HSPEVENT_MAX: i32 = 17;

pub const GINFO_EXINFO_MAX: i32 = 16;
pub const GINFO_EXINFO_BASE: i32 = 0x100;
pub const GINFO_EXINFO_ACCEL_X: i32 = GINFO_EXINFO_BASE + 0;
pub const GINFO_EXINFO_ACCEL_Y: i32 = GINFO_EXINFO_BASE + 1;
pub const GINFO_EXINFO_ACCEL_Z: i32 = GINFO_EXINFO_BASE + 2;
pub const GINFO_EXINFO_GYRO_X: i32 = GINFO_EXINFO_BASE + 3;
pub const GINFO_EXINFO_GYRO_Y: i32 = GINFO_EXINFO_BASE + 4;
pub const GINFO_EXINFO_GYRO_Z: i32 = GINFO_EXINFO_BASE + 5;
