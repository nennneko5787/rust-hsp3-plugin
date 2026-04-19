#![allow(non_camel_case_types, non_snake_case, dead_code)]

use crate::*;

pub const HSPVAR_FLAG_NONE: i32 = 0;
pub const HSPVAR_FLAG_LABEL: i32 = 1;
pub const HSPVAR_FLAG_STR: i32 = 2;
pub const HSPVAR_FLAG_DOUBLE: i32 = 3;
pub const HSPVAR_FLAG_INT: i32 = 4;
pub const HSPVAR_FLAG_STRUCT: i32 = 5;
pub const HSPVAR_FLAG_COMSTRUCT: i32 = 6;
pub const HSPVAR_FLAG_USERDEF: i32 = 8;
pub const HSPVAR_FLAG_MAX: i32 = 8;

pub const HSPVAR_MODE_NONE: i32 = -1;
pub const HSPVAR_MODE_MALLOC: i32 = 1;
pub const HSPVAR_MODE_CLONE: i32 = 2;

pub const HSPVAR_ERROR_INVALID: HSPERROR = HSPERROR_HSPERR_WRONG_EXPRESSION;
pub const HSPVAR_ERROR_DIVZERO: HSPERROR = HSPERROR_HSPERR_DIVIDED_BY_ZERO;
pub const HSPVAR_ERROR_TYPEMISS: HSPERROR = HSPERROR_HSPERR_TYPE_MISMATCH;
pub const HSPVAR_ERROR_ARRAYOVER: HSPERROR = HSPERROR_HSPERR_ARRAY_OVERFLOW;
pub const HSPVAR_ERROR_ILLEGALPRM: HSPERROR = HSPERROR_HSPERR_ILLEGAL_FUNCTION;

pub const HSPVAR_SUPPORT_STORAGE: u16 = 1;
pub const HSPVAR_SUPPORT_FLEXSTORAGE: u16 = 2;
pub const HSPVAR_SUPPORT_FIXEDARRAY: u16 = 4;
pub const HSPVAR_SUPPORT_FLEXARRAY: u16 = 8;
pub const HSPVAR_SUPPORT_ARRAYOBJ: u16 = 16;
pub const HSPVAR_SUPPORT_FLEXSIZE: u16 = 32;
pub const HSPVAR_SUPPORT_NOCONVERT: u16 = 64;
pub const HSPVAR_SUPPORT_VARUSE: u16 = 128;
pub const HSPVAR_SUPPORT_TEMPVAR: u16 = 256;
pub const HSPVAR_SUPPORT_USER1: u16 = 0x4000;
pub const HSPVAR_SUPPORT_USER2: u16 = 0x8000;
pub const HSPVAR_SUPPORT_MISCTYPE: u16 = HSPVAR_SUPPORT_ARRAYOBJ;

pub type PDAT = *mut ::std::ffi::c_void;
pub type APTR = i32;

pub const CALCCODE_ADD: i32 = 0;
pub const CALCCODE_SUB: i32 = 1;
pub const CALCCODE_MUL: i32 = 2;
pub const CALCCODE_DIV: i32 = 3;
pub const CALCCODE_MOD: i32 = 4;
pub const CALCCODE_AND: i32 = 5;
pub const CALCCODE_OR: i32 = 6;
pub const CALCCODE_XOR: i32 = 7;
pub const CALCCODE_EQ: i32 = 8;
pub const CALCCODE_NE: i32 = 9;
pub const CALCCODE_GT: i32 = 10;
pub const CALCCODE_LT: i32 = 11;
pub const CALCCODE_GTEQ: i32 = 12;
pub const CALCCODE_LTEQ: i32 = 13;
pub const CALCCODE_RR: i32 = 14;
pub const CALCCODE_LR: i32 = 15;
pub const CALCCODE_MAX: i32 = 16;

#[repr(C)]
pub struct PVal {
    pub flag: i16,
    pub mode: i16,
    pub len: [i32; 5],
    pub size: i32,
    pub pt: *mut i8,
    pub master: *mut ::std::ffi::c_void,
    pub support: u16,
    pub arraycnt: i16,
    pub offset: i32,
    pub arraymul: i32,
}

pub type HSPVAR_COREFUNC = Option<unsafe extern "C" fn(*mut HspVarProc)>;

#[repr(C)]
pub struct HspVarProc {
    pub flag: i16,
    pub aftertype: i16,
    pub version: i16,
    pub support: u16,
    pub basesize: i16,
    pub opt: i16,
    pub vartype_name: *mut i8,
    pub user: *mut i8,
    pub Cnv: Option<unsafe extern "C" fn(*const ::std::ffi::c_void, i32) -> *mut ::std::ffi::c_void>,
    pub CnvCustom:
        Option<unsafe extern "C" fn(*const ::std::ffi::c_void, i32) -> *mut ::std::ffi::c_void>,
    pub GetPtr: Option<unsafe extern "C" fn(*mut PVal) -> *mut PDAT>,
    pub ArrayObjectRead:
        Option<unsafe extern "C" fn(*mut PVal, *mut i32) -> *mut ::std::ffi::c_void>,
    pub ArrayObject: Option<unsafe extern "C" fn(*mut PVal)>,
    pub ObjectWrite: Option<unsafe extern "C" fn(*mut PVal, *mut ::std::ffi::c_void, i32)>,
    pub ObjectMethod: Option<unsafe extern "C" fn(*mut PVal)>,
    pub Alloc: Option<unsafe extern "C" fn(*mut PVal, *const PVal)>,
    pub Free: Option<unsafe extern "C" fn(*mut PVal)>,
    pub GetSize: Option<unsafe extern "C" fn(*const PDAT) -> i32>,
    pub GetUsing: Option<unsafe extern "C" fn(*const PDAT) -> i32>,
    pub GetBlockSize:
        Option<unsafe extern "C" fn(*mut PVal, PDAT, *mut i32) -> *mut ::std::ffi::c_void>,
    pub AllocBlock: Option<unsafe extern "C" fn(*mut PVal, PDAT, i32)>,
    pub Set: Option<unsafe extern "C" fn(*mut PVal, PDAT, *const ::std::ffi::c_void)>,
    pub AddI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub SubI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub MulI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub DivI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub ModI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub AndI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub OrI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub XorI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub EqI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub NeI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub GtI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub LtI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub GtEqI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub LtEqI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub RrI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
    pub LrI: Option<unsafe extern "C" fn(PDAT, *const ::std::ffi::c_void)>,
}

unsafe extern "C" {
    pub static mut hspvarproc: *mut HspVarProc;
    pub static mut hspvartype_max: i32;
    pub static mut mem_pval: *mut PVal;

    pub fn HspVarCoreInit();
    pub fn HspVarCoreBye();
    pub fn HspVarCoreResetVartype(expand: i32);
    pub fn HspVarCoreAddType() -> i32;
    pub fn HspVarCoreRegisterType(flag: i32, func: HSPVAR_COREFUNC);
    pub fn HspVarCoreSeekProc(name: *const i8) -> *mut HspVarProc;
    pub fn HspVarCoreDup(pval: *mut PVal, arg: *mut PVal, aptr: APTR);
    pub fn HspVarCoreDupPtr(pval: *mut PVal, flag: i32, ptr: *mut ::std::ffi::c_void, size: i32);
    pub fn HspVarCoreClear(pval: *mut PVal, flag: i32);
    pub fn HspVarCoreClearTemp(pval: *mut PVal, flag: i32);
    pub fn HspVarCoreDim(pval: *mut PVal, flag: i32, len1: i32, len2: i32, len3: i32, len4: i32);
    pub fn HspVarCoreDimFlex(
        pval: *mut PVal,
        flag: i32,
        len0: i32,
        len1: i32,
        len2: i32,
        len3: i32,
        len4: i32,
    );
    pub fn HspVarCoreReDim(pval: *mut PVal, lenid: i32, len: i32);
    pub fn HspVarCoreCnvPtr(pval: *mut PVal, flag: i32) -> *mut ::std::ffi::c_void;
    pub fn HspVarCorePtrAPTR(pv: *mut PVal, ofs: APTR) -> *mut PDAT;
    pub fn HspVarCoreArray(pval: *mut PVal, offset: i32);
}

pub const FLEXVAL_TYPE_NONE: i32 = 0;
pub const FLEXVAL_TYPE_ALLOC: i32 = 1;
pub const FLEXVAL_TYPE_CLONE: i32 = 2;

#[repr(C)]
pub struct FlexValue {
    pub type_: i16,
    pub myid: i16,
    pub customid: i16,
    pub clonetype: i16,
    pub size: i32,
    pub ptr: *mut ::std::ffi::c_void,
}
