use hsp3sdk::hsp3plugin::*;
use hsp3sdk::hsp3struct::*;
use hsp3sdk::hspvar_core::*;
use hsp3sdk::hspwnd::*;
use hsp3sdk::{
    BOOL, HINSTANCE, HSPERROR_HSPERR_DIVIDED_BY_ZERO, HSPERROR_HSPERR_FILE_IO,
    HSPERROR_HSPERR_INVALID_FUNCPARAM, HSPERROR_HSPERR_TYPE_MISMATCH,
    HSPERROR_HSPERR_UNSUPPORTED_FUNCTION,
};
use std::ffi::CStr;
use std::fs::OpenOptions;
use std::io::Write;
use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static mut REF_IVAL: i32 = 0;
static RNG_STATE: AtomicU32 = AtomicU32::new(0);

fn next_random() -> u32 {
    let seed = RNG_STATE.load(Ordering::Relaxed);
    let state = if seed == 0 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(1)
    } else {
        seed
    };
    let next = state.wrapping_mul(1664525).wrapping_add(1013904223);
    RNG_STATE.store(next, Ordering::Relaxed);
    next
}

unsafe fn newcmd2() {
    unsafe {
        let mut pval: *mut PVal = ptr::null_mut();
        let aptr = (*exinfo).HspFunc_prm_getva.unwrap()(&mut pval);
        P1 = (*exinfo).HspFunc_prm_getdi.unwrap()(100);
        if P1 == 0 {
            (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_DIVIDED_BY_ZERO);
            return;
        }
        P2 = (next_random() % (P1 as u32)) as i32;
        (*exinfo).HspFunc_prm_setva.unwrap()(
            pval,
            aptr,
            HSPVAR_FLAG_INT,
            &raw const P2 as *const _ as *const _,
        );
    }
}

unsafe fn newcmd3() {
    unsafe {
        let p = (*exinfo).HspFunc_prm_gets.unwrap()();
        if p.is_null() {
            return;
        }

        let pathname = match CStr::from_ptr(p).to_str() {
            Ok(s) => s.to_owned(),
            Err(_) => {
                (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_TYPE_MISMATCH);
                return;
            }
        };

        let chk = code_getprm();
        if chk <= PARAM_END {
            return;
        }

        let out = match (*mpval).flag as i32 {
            HSPVAR_FLAG_STR => {
                let s = CStr::from_ptr((*mpval).pt).to_string_lossy();
                format!("{pathname}\n{s}\n")
            }
            HSPVAR_FLAG_DOUBLE => {
                let ptr = (*mpval).pt as *const f64;
                format!("{pathname}\n{}\n", *ptr)
            }
            HSPVAR_FLAG_INT => {
                let ptr = (*mpval).pt as *const i32;
                format!("{pathname}\n{}\n", *ptr)
            }
            _ => {
                (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_TYPE_MISMATCH);
                return;
            }
        };

        let mut file = match OpenOptions::new().create(true).append(true).open(&pathname) {
            Ok(f) => f,
            Err(_) => {
                (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_FILE_IO);
                return;
            }
        };

        if file.write_all(out.as_bytes()).is_err() {
            (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_FILE_IO);
        }
    }
}

unsafe fn newcmd4() {
    unsafe {
        P1 = (*exinfo).HspFunc_prm_getdi.unwrap()(0);
        P2 = (*exinfo).HspFunc_prm_getdi.unwrap()(0);
        P3 = (*exinfo).HspFunc_prm_getdi.unwrap()(0);
        P4 = (*exinfo).HspFunc_prm_getdi.unwrap()(0);
        P5 = (*exinfo).HspFunc_prm_getdi.unwrap()(0);

        let bm = (*exinfo).HspFunc_getbmscr.unwrap()(*(*exinfo).actscr) as *mut BMSCR;
        if bm.is_null() {
            return;
        }

        draw_line(bm, P1, P2, P3, P4, P5);
        bms_send(bm, 0, 0, (*bm).sx, (*bm).sy);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cmdfunc(cmd: i32) -> i32 {
    unsafe {
        (*exinfo).HspFunc_prm_next.unwrap()();

        match cmd {
            0x00 => {
                P1 = (*exinfo).HspFunc_prm_getdi.unwrap()(123);
                (*ctx).stat = P1;
            }
            0x01 => newcmd2(),
            0x02 => newcmd3(),
            0x03 => newcmd4(),
            _ => (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_UNSUPPORTED_FUNCTION),
        }

        RUNMODE_RUN
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reffunc(type_res: *mut i32, cmd: i32) -> *mut ::core::ffi::c_void {
    unsafe {
        if *TYPE_ != TYPE_MARK {
            (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_INVALID_FUNCPARAM);
        }
        if *VAL != '(' as i32 {
            (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_INVALID_FUNCPARAM);
        }
        (*exinfo).HspFunc_prm_next.unwrap()();

        match cmd {
            0x00 => {
                P1 = (*exinfo).HspFunc_prm_geti.unwrap()();
                REF_IVAL = P1 * 2;
            }
            _ => {
                (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_UNSUPPORTED_FUNCTION);
            }
        }

        if *TYPE_ != TYPE_MARK {
            (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_INVALID_FUNCPARAM);
        }
        if *VAL != ')' as i32 {
            (*exinfo).HspFunc_puterror.unwrap()(HSPERROR_HSPERR_INVALID_FUNCPARAM);
        }
        (*exinfo).HspFunc_prm_next.unwrap()();

        *type_res = HSPVAR_FLAG_INT;
        &raw mut REF_IVAL as *mut _ as *mut ::core::ffi::c_void
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn termfunc(_: i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn eventfunc(event: i32, _: i32, _: i32, prm3: *mut ::core::ffi::c_void) -> i32 {
    unsafe {
        match event {
            HSPEVENT_GETKEY => {
                let ival = prm3 as *mut i32;
                if !ival.is_null() {
                    *ival = 123;
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn DllMain(_: HINSTANCE, _: u32, _: *mut ::core::ffi::c_void) -> BOOL {
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hsp3cmdinit(info: *mut HSP3TYPEINFO) {
    unsafe {
        hsp3sdk_init(info);
        (*info).cmdfunc = Some(cmdfunc);
        (*info).reffunc = Some(reffunc);
        (*info).termfunc = Some(termfunc);
    }
}

pub unsafe fn draw_line(bm: *mut BMSCR, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) {
    unsafe {
        let lp_dest = (*bm).pBit as *mut u8;
        let n_dest_width = (*bm).sx;
        let n_dest_wbyte = (*bm).sx * 3;
        let n_dest_height = (*bm).sy;
        let dw_dest = lp_dest.add(((n_dest_height - 1) * n_dest_wbyte) as usize);

        let color1 = (color & 0xffff) as u16;
        let color2 = ((color & 0xff0000) >> 16) as u8;

        let mut dx = x2 - x1;
        let addx = if dx < 0 {
            dx = -dx;
            -1
        } else {
            1
        };

        let mut dy = y2 - y1;
        let addy = if dy < 0 {
            dy = -dy;
            -1
        } else {
            1
        };

        let mut x = x1;
        let mut y = y1;
        let mut cnt = 0;

        if dx > dy {
            for _ in 0..dx {
                if y >= 0 && y < n_dest_height && x >= 0 && x < n_dest_width {
                    let up = dw_dest
                        .offset(-(n_dest_wbyte * y) as isize)
                        .add((x * 3) as usize);
                    *(up as *mut u16) = color1;
                    *up.add(2) = color2;
                }
                cnt += dy;
                x += addx;
                if cnt >= dx {
                    cnt -= dx;
                    y += addy;
                }
            }
        } else {
            for _ in 0..dy {
                if y >= 0 && y < n_dest_height && x >= 0 && x < n_dest_width {
                    let up = dw_dest
                        .offset(-(n_dest_wbyte * y) as isize)
                        .add((x * 3) as usize);
                    *(up as *mut u16) = color1;
                    *up.add(2) = color2;
                }
                cnt += dx;
                y += addy;
                if cnt >= dy {
                    cnt -= dy;
                    x += addx;
                }
            }
        }
    }
}
