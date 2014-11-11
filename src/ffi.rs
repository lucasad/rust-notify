#![allow(improper_ctypes)]

extern crate libc;
use std::c_str;

#[repr(C)]
pub struct Notification;

#[repr(C)]
pub struct Error {
    pub domain: u32,
    pub code: i32,
    pub message: c_str::CString
}
    
#[link(name = "notify")]
extern "C" {
    pub fn notify_init(app_name: *const i8) -> bool;
    pub fn notify_uninit();
    pub fn notify_is_initted() -> bool;

    pub fn notify_notification_new(summary: *const i8, body: *const i8, icon: *const i8) -> *mut Notification;
    pub fn notify_notification_change(notification: *mut Notification, summary: *const i8, body: *const i8, icon: *const i8, error: *mut Error) -> bool;
    pub fn notify_notification_set_urgency(notification: *mut Notification, urgency: i32);
    pub fn notify_notification_set_timeout(notification: *mut Notification, timeout: i64);
    pub fn notify_notification_set_category(notification: *mut Notification, category: *const i8);
    pub fn notify_notification_show(notification: *mut Notification, error: *mut Error) -> bool;
    pub fn notify_notification_close(notification: *mut Notification, error: *mut Error) -> bool;
}

#[link(name = "glib-2.0")]
extern {
    pub fn g_quark_to_string(quark: u32) -> c_str::CString;
}
