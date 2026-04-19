use crate::*;

pub static mut P1: ::std::os::raw::c_int = 0;
pub static mut P2: ::std::os::raw::c_int = 0;
pub static mut P3: ::std::os::raw::c_int = 0;
pub static mut P4: ::std::os::raw::c_int = 0;
pub static mut P5: ::std::os::raw::c_int = 0;
pub static mut P6: ::std::os::raw::c_int = 0;
pub static mut TYPE_: *mut ::std::os::raw::c_int = std::ptr::null_mut();
pub static mut VAL: *mut ::std::os::raw::c_int = std::ptr::null_mut();
pub static mut mpval: *mut PVal = std::ptr::null_mut();
pub static mut ctx: *mut HSPCTX = std::ptr::null_mut();
pub static mut exinfo: *mut HSPEXINFO = std::ptr::null_mut();

pub unsafe fn hsp3sdk_init(info: *mut HSP3TYPEINFO) { unsafe {
    ctx = (*info).hspctx;
    exinfo = (*info).hspexinfo;
    TYPE_ = (*exinfo).nptype;
    VAL = (*exinfo).npval;
}}

pub unsafe fn code_getprm() -> ::std::os::raw::c_int { unsafe {
    let res = (*exinfo).HspFunc_prm_get.unwrap()();
    mpval = *(*exinfo).mpval;
    res
}}

pub unsafe fn bms_send(bm: *mut BMSCR, x: i32, y: i32, sx: i32, sy: i32) { unsafe {
    if (*bm).fl_udraw == 0 {
        return;
    }
    let hdc = GetDC((*bm).hwnd);
    let opal = if !(*bm).hpal.is_null() {
        let opal = SelectPalette(hdc, (*bm).hpal, 0);
        RealizePalette(hdc);
        Some(opal)
    } else {
        None
    };
    BitBlt(
        hdc,
        x - (*bm).viewx,
        y - (*bm).viewy,
        sx,
        sy,
        (*bm).hdc,
        x,
        y,
        SRCCOPY,
    );
    if let Some(opal) = opal {
        SelectPalette(hdc, opal, 0);
    }
    ReleaseDC((*bm).hwnd, hdc);
}}

#[macro_export]
macro_rules! code_next {
    () => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_next.unwrap()() }
    };
}

#[macro_export]
macro_rules! puterror {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_puterror.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! code_event {
    ($event:expr, $prm1:expr, $prm2:expr, $prm3:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_hspevent.unwrap()($event, $prm1, $prm2, $prm3) }
    };
}

#[macro_export]
macro_rules! code_geti {
    () => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_geti.unwrap()() }
    };
}

#[macro_export]
macro_rules! code_getdi {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_getdi.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! code_gets {
    () => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_gets.unwrap()() }
    };
}

#[macro_export]
macro_rules! code_getds {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_getds.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! getbmscr {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_getbmscr.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! code_getlb {
    () => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_getlb.unwrap()() }
    };
}

#[macro_export]
macro_rules! code_getpval {
    () => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_getpval.unwrap()() }
    };
}

#[macro_export]
macro_rules! code_getva {
    ($pval:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_getva.unwrap()($pval) }
    };
}

#[macro_export]
macro_rules! code_setva {
    ($p:expr, $a:expr, $t:expr, $v:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_setva.unwrap()($p, $a, $t, $v) }
    };
}

#[macro_export]
macro_rules! hspmalloc {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_malloc.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! hspfree {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_free.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! hspexpand {
    ($p:expr, $s:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_expand.unwrap()($p, $s) }
    };
}

#[macro_export]
macro_rules! code_getd {
    () => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_getd.unwrap()() }
    };
}

#[macro_export]
macro_rules! code_getdd {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_getdd.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! code_getns {
    () => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_getns.unwrap()() }
    };
}

#[macro_export]
macro_rules! code_getnds {
    ($p:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_prm_getnds.unwrap()($p) }
    };
}

#[macro_export]
macro_rules! stat {
    () => {
        unsafe { (*$crate::hsp3sdk::ctx).stat }
    };
    ($val:expr) => {
        unsafe { (*$crate::hsp3sdk::ctx).stat = $val }
    };
}

#[macro_export]
macro_rules! getobj {
    ($wid:expr, $id:expr, $inf:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_getobj.unwrap()($wid, $id, $inf) }
    };
}

#[macro_export]
macro_rules! setobj {
    ($wid:expr, $id:expr, $inf:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_setobj.unwrap()($wid, $id, $inf) }
    };
}

#[macro_export]
macro_rules! addobj {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_addobj.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! getproc {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_getproc.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! seekproc {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_seekproc.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! addirq {
    () => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_addirq.unwrap()() }
    };
}

#[macro_export]
macro_rules! registvar {
    ($x:expr, $y:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_registvar.unwrap()($x, $y) }
    };
}

#[macro_export]
macro_rules! code_setpc {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_setpc.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! code_call {
    ($x:expr) => {
        unsafe { (*$crate::hsp3sdk::exinfo).HspFunc_call.unwrap()($x) }
    };
}

#[macro_export]
macro_rules! active_window {
    () => {
        unsafe { *(*$crate::hsp3sdk::exinfo).actscr }
    };
}
