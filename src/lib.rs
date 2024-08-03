/*
 * Copyright 2024 Oxide Computer Company
 */

#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::marker::{PhantomData, PhantomPinned};
use std::os::raw::{c_char, c_int, c_uint};

#[cfg(feature = "private")]
use libc::c_void;
use libc::{ctid_t, id_t, pid_t, size_t, zoneid_t};
use num_derive::{FromPrimitive, ToPrimitive};

macro_rules! opaque_handle {
    ($type_name:ident) => {
        #[repr(C)]
        pub struct $type_name {
            _data: [u8; 0],
            /*
             * See https://doc.rust-lang.org/nomicon/ffi.html; this marker
             * guarantees our type does not implement "Send", "Sync", or
             * "Unpin".
             */
            _marker: PhantomData<(*mut u8, PhantomPinned)>,
        }
        impl Copy for $type_name {}
        impl Clone for $type_name {
            fn clone(&self) -> $type_name {
                *self
            }
        }
    };
}

opaque_handle!(ct_stathdl_t);
opaque_handle!(ct_evthdl_t);

pub type ctevid_t = u64;

/*
 * Values for the "detail" argument of ct_status_read(3CONTRACT):
 */
pub const CTD_COMMON: c_int = 0;
pub const CTD_FIXED: c_int = 1;
pub const CTD_ALL: c_int = 2;

/*
 * Common event types; see ct_event_get_type(3CONTRACT) and contract(5):
 */
pub const CT_EV_NEGEND: c_uint = 0;

/*
 * Contract parameter maximum size, in bytes:
 */
pub const CT_PARAM_MAX_SIZE: usize = 8192;

/*
 * Values returned by ct_event_get_flags(3CONTRACT):
 */
pub const CTE_ACK: c_uint = 0x1;
pub const CTE_INFO: c_uint = 0x2;
pub const CTE_NEG: c_uint = 0x4;

#[derive(Debug, FromPrimitive, ToPrimitive, Clone, Copy)]
#[repr(C)]
pub enum ctstate_t {
    CTS_OWNED,
    CTS_INHERITED,
    CTS_ORPHAN,
    CTS_DEAD,
}

#[derive(Debug, FromPrimitive, ToPrimitive, Clone, Copy)]
#[repr(C)]
pub enum ct_typeid_t {
    CTT_PROCESS,
    CTT_DEVICE,
}

#[cfg(feature = "private")]
#[derive(Debug)]
#[repr(C)]
pub struct ct_event_t {
    pub ctev_id: ctid_t,
    pub ctev_pad1: u32,
    pub ctev_evid: ctevid_t,
    pub ctev_cttype: ct_typeid_t,
    pub ctev_flags: u32,
    pub ctev_type: u32,
    pub ctev_nbytes: u32,
    pub ctev_goffset: u32,
    pub ctev_pad2: u32,
    pub ctev_buffer: *mut c_char,
}

#[cfg(feature = "private")]
#[derive(Debug)]
#[repr(C)]
pub struct ct_status_t {
    pub ctst_id: ctid_t,
    pub ctst_zoneid: zoneid_t,
    pub ctst_type: ct_typeid_t,
    pub ctst_holder: pid_t,
    pub ctst_state: ctstate_t,
    pub ctst_nevents: c_int,
    pub ctst_ntime: c_int,
    pub ctst_qtime: c_int,
    pub ctst_nevid: u64,
    pub ctst_detail: c_uint,
    pub ctst_nbytes: size_t,
    pub ctst_critical: c_uint,
    pub ctst_informative: c_uint,
    pub ctst_cookie: u64,
    pub ctst_buffer: *mut c_char,
}

#[cfg(feature = "private")]
#[derive(Debug)]
#[repr(C)]
pub struct ct_param_t {
    pub ctpm_id: u32,
    pub ctpm_size: u32,
    pub ctpm_value: *mut c_void,
}

#[link(name = "contract")]
extern "C" {
    /*
     * Common contract template functions:
     */

    pub fn ct_tmpl_activate(fd: c_int) -> c_int;
    pub fn ct_tmpl_clear(fd: c_int) -> c_int;
    pub fn ct_tmpl_create(fd: c_int, idp: *mut ctid_t) -> c_int;

    pub fn ct_tmpl_set_cookie(fd: c_int, cookie: u64) -> c_int;
    pub fn ct_tmpl_set_critical(fd: c_int, events: c_uint) -> c_int;
    pub fn ct_tmpl_set_informative(fd: c_int, events: c_uint) -> c_int;

    pub fn ct_tmpl_get_cookie(fd: c_int, cookiep: *mut u64) -> c_int;
    pub fn ct_tmpl_get_critical(fd: c_int, eventsp: *mut c_uint) -> c_int;
    pub fn ct_tmpl_get_informative(fd: c_int, eventsp: *mut c_uint) -> c_int;

    /*
     * Common contract control functions:
     */

    pub fn ct_ctl_adopt(fd: c_int) -> c_int;
    pub fn ct_ctl_abandon(fd: c_int) -> c_int;
    pub fn ct_ctl_newct(fd: c_int, evid: ctevid_t, templatefd: c_int) -> c_int;

    pub fn ct_ctl_ack(fd: c_int, evid: ctevid_t) -> c_int;
    pub fn ct_ctl_nack(fd: c_int, evid: ctevid_t) -> c_int;
    pub fn ct_ctl_qack(fd: c_int, evid: ctevid_t) -> c_int;

    /*
     * Common contract status functions:
     */

    pub fn ct_status_read(
        fd: c_int,
        detail: c_int,
        stathdlp: *mut *mut ct_stathdl_t,
    ) -> c_int;
    pub fn ct_status_free(stathdl: *mut ct_stathdl_t);

    pub fn ct_status_get_id(stathdl: *mut ct_stathdl_t) -> ctid_t;
    pub fn ct_status_get_zoneid(stathdl: ct_stathdl_t) -> zoneid_t;
    pub fn ct_status_get_type(stathdl: ct_stathdl_t) -> *const c_char;
    pub fn ct_status_get_state(stathdl: ct_stathdl_t) -> ctstate_t;
    pub fn ct_status_get_holder(stathdl: ct_stathdl_t) -> id_t;
    pub fn ct_status_get_nevents(stathdl: ct_stathdl_t) -> c_int;
    pub fn ct_status_get_ntime(stathdl: ct_stathdl_t) -> c_int;
    pub fn ct_status_get_qtime(stathdl: ct_stathdl_t) -> c_int;
    pub fn ct_status_get_nevid(stathdl: ct_stathdl_t) -> ctevid_t;
    pub fn ct_status_get_cookie(stathdl: ct_stathdl_t) -> u64;
    pub fn ct_status_get_informative(stathdl: ct_stathdl_t) -> c_uint;
    pub fn ct_status_get_critical(stathdl: ct_stathdl_t) -> c_uint;

    /*
     * Common contract event functions:
     */

    pub fn ct_event_read(fd: c_int, evthndlp: *mut *mut ct_evthdl_t) -> c_int;
    pub fn ct_event_read_critical(
        fd: c_int,
        evthandlp: *mut *mut ct_evthdl_t,
    ) -> c_int;
    pub fn ct_event_reset(fd: c_int) -> c_int;
    pub fn ct_event_reliable(fd: c_int) -> c_int;
    pub fn ct_event_free(evthndl: *mut ct_evthdl_t);

    pub fn ct_event_get_ctid(evthndl: *mut ct_evthdl_t) -> ctid_t;
    pub fn ct_event_get_evid(evthndl: *mut ct_evthdl_t) -> ctevid_t;
    pub fn ct_event_get_flags(evthndl: *mut ct_evthdl_t) -> c_uint;
    pub fn ct_event_get_type(evthndl: *mut ct_evthdl_t) -> c_uint;
    pub fn ct_event_get_nevid(
        evthndl: *mut ct_evthdl_t,
        evidp: *mut ctevid_t,
    ) -> c_int;
    pub fn ct_event_get_newct(
        evthndl: *mut ct_evthdl_t,
        ctidp: *mut ctid_t,
    ) -> c_int;

    /*
     * Process contract template functions:
     */

    pub fn ct_pr_tmpl_set_transfer(fd: c_int, ctid: ctid_t) -> c_int;
    pub fn ct_pr_tmpl_set_fatal(fd: c_int, events: c_uint) -> c_int;
    pub fn ct_pr_tmpl_set_param(fd: c_int, params: c_uint) -> c_int;
    pub fn ct_pr_tmpl_set_svc_fmri(fd: c_int, fmri: *const c_char) -> c_int;
    pub fn ct_pr_tmpl_set_svc_aux(fd: c_int, aux: *const c_char) -> c_int;

    pub fn ct_pr_tmpl_get_transfer(fd: c_int, ctidp: *mut ctid_t) -> c_int;
    pub fn ct_pr_tmpl_get_fatal(fd: c_int, eventsp: *mut c_uint) -> c_int;
    pub fn ct_pr_tmpl_get_param(fd: c_int, eventsp: *mut c_uint) -> c_int;
    pub fn ct_pr_tmpl_get_svc_fmri(
        fd: c_int,
        fmri: *mut c_char,
        size: size_t,
    ) -> c_int;
    pub fn ct_pr_tmpl_get_svc_aux(
        fd: c_int,
        aux: *mut c_char,
        size: size_t,
    ) -> c_int;

    /*
     * Process contract event functions:
     */

    pub fn ct_pr_event_get_pid(evthdl: ct_evthdl_t, pidp: *mut pid_t) -> c_int;
    pub fn ct_pr_event_get_ppid(evthdl: ct_evthdl_t, pidp: *mut pid_t)
        -> c_int;
    pub fn ct_pr_event_get_signal(
        evthdl: ct_evthdl_t,
        signalp: *mut c_int,
    ) -> c_int;
    pub fn ct_pr_event_get_sender(
        evthdl: ct_evthdl_t,
        pidp: *mut pid_t,
    ) -> c_int;
    pub fn ct_pr_event_get_senderct(
        evthdl: ct_evthdl_t,
        ctidp: *mut ctid_t,
    ) -> c_int;
    pub fn ct_pr_event_get_exitstatus(
        evthdl: ct_evthdl_t,
        statusp: *mut c_int,
    ) -> c_int;
    pub fn ct_pr_event_get_pcorefile(
        evthdl: ct_evthdl_t,
        namep: *mut *mut c_char,
    ) -> c_int;
    pub fn ct_pr_event_get_gcorefile(
        evthdl: ct_evthdl_t,
        namep: *mut *mut c_char,
    ) -> c_int;
    pub fn ct_pr_event_get_zcorefile(
        evthdl: ct_evthdl_t,
        namep: *mut *mut c_char,
    ) -> c_int;

    /*
     * Process contract status functions:
     */

    pub fn ct_pr_status_get_param(
        stathdl: *const ct_stathdl_t,
        paramp: *mut c_uint,
    ) -> c_int;
    pub fn ct_pr_status_get_fatal(
        stathdl: *const ct_stathdl_t,
        eventsp: *mut c_uint,
    ) -> c_int;
    pub fn ct_pr_status_get_members(
        stathdl: *const ct_stathdl_t,
        pidpp: *mut *mut pid_t,
        n: *mut c_uint,
    ) -> c_int;
    pub fn ct_pr_status_get_contracts(
        stathdl: *const ct_stathdl_t,
        idpp: *mut *mut ctid_t,
        n: *mut c_uint,
    ) -> c_int;
    pub fn ct_pr_status_get_svc_fmri(
        stathdl: *const ct_stathdl_t,
        fmri: *mut *mut c_char,
    ) -> c_int;
    pub fn ct_pr_status_get_svc_aux(
        stathdl: *const ct_stathdl_t,
        aux: *mut *mut c_char,
    ) -> c_int;
    pub fn ct_pr_status_get_svc_ctid(
        stathdl: *const ct_stathdl_t,
        ctid: *mut ctid_t,
    ) -> c_int;
    pub fn ct_pr_status_get_svc_creator(
        stathdl: *const ct_stathdl_t,
        creator: *mut *mut c_char,
    ) -> c_int;

    /*
     * Device contract template functions:
     */

    pub fn ct_dev_tmpl_set_aset(fd: c_int, aset: c_uint) -> c_int;
    pub fn ct_dev_tmpl_get_aset(fd: c_int, asetp: *mut c_uint) -> c_int;

    pub fn ct_dev_tmpl_set_minor(fd: c_int, minor: *const c_char) -> c_int;
    pub fn ct_dev_tmpl_get_minor(
        fd: c_int,
        buf: *mut c_char,
        buflenp: *mut size_t,
    ) -> c_int;

    pub fn ct_dev_tmpl_set_noneg(fd: c_int) -> c_int;
    pub fn ct_dev_tmpl_clear_noneg(fd: c_int) -> c_int;
    pub fn ct_dev_tmpl_get_noneg(fd: c_int, nonegp: *mut c_uint) -> c_int;

    /*
     * Device contract status functions:
     */

    pub fn ct_dev_status_get_dev_state(
        stathdl: *const ct_stathdl_t,
        statep: *mut c_uint,
    ) -> c_int;
    pub fn ct_dev_status_get_aset(
        stathdl: *const ct_stathdl_t,
        asetp: *mut c_uint,
    ) -> c_int;
    pub fn ct_dev_status_get_minor(
        stathdl: *const ct_stathdl_t,
        minorp: *mut *mut c_char,
    ) -> c_int;
    pub fn ct_dev_status_get_noneg(
        stathdl: *const ct_stathdl_t,
        nonegp: *mut c_uint,
    ) -> c_int;
}
