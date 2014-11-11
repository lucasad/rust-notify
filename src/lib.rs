extern crate libc;
use std::result::Result;

mod ffi;

pub fn init(name: &str) -> bool{
    let name_c_string = name.to_c_str();
    unsafe {
        return ffi::notify_init(name_c_string.as_ptr());
    }
}

pub fn uninit() {
    unsafe {
        ffi::notify_uninit();
    }
}

pub fn is_initted() -> bool {
    unsafe {
        return ffi::notify_is_initted();
    }
}

pub struct Error {
    domain: String,
    code: i32,
    message: String
}

impl Error {
    fn new(error: ffi::Error) -> ::Error {
        unsafe {
            let domain: String = ffi::g_quark_to_string(error.domain).to_string();
            let message: String = error.message.to_string();
            return Error { domain:domain, code:error.code, message:message }
        }
    }
}

pub enum Urgency {
    Low,
    Normal,
    Critical
}

pub struct Notification {
    ptr: *mut ffi::Notification
}

impl Notification {
    pub fn new(summary: &str, body: &str, icon: &str) -> Notification {
        let summary_c = summary.to_c_str();
        let body_c = body.to_c_str();
        let icon_c = icon.to_c_str();
        unsafe {
            return Notification { ptr: ffi::notify_notification_new(summary_c.as_ptr(), body_c.as_ptr(), icon_c.as_ptr()) }
        }
    }

    pub fn show(&self) -> Result<(),Error> {
        unsafe {
            let mut error: ffi::Error = std::mem::zeroed();
            return
                if ffi::notify_notification_show(self.ptr, &mut error) {
                    Ok(())
                }
                else {
                    println!("Error showing...");
                    Err(Error::new(error))
                }
        }
    }

    pub fn set_urgency(&self, urgency: Urgency) {
        let urgency = match urgency {
            Low => 0,
            Normal => 1,
            Critical => 2
        };
        unsafe {
            ffi::notify_notification_set_urgency(self.ptr, urgency);
        }
    }

    pub fn set_timeout(&self, duration: std::time::duration::Duration) {
        let ms = duration.num_milliseconds();
        unsafe {
            ffi::notify_notification_set_timeout(self.ptr, ms);
        }
    }

    pub fn set_category(&self, category: &str) {
        let category = category.to_c_str();
        unsafe {
            ffi::notify_notification_set_category(self.ptr, category.as_ptr());
        }
    }

    pub fn change(&self, summary: &str, body: &str, icon: &str) -> Result<(),Error> {
        let summary = summary.to_c_str();
        let body = body.to_c_str();
        let icon = icon.to_c_str();
        unsafe {
            let mut error: ffi::Error = std::mem::zeroed();
            return
                if ffi::notify_notification_change(self.ptr, summary.as_ptr(), body.as_ptr(), icon.as_ptr(), &mut error) {
                    Ok(())
                } else {
                    Err(Error::new(error))
                }
        }
    }
    pub fn close(&self) -> Result<(), Error> {
        unsafe {
            let mut error: ffi::Error = std::mem::zeroed();
            return
                if ffi::notify_notification_close(self.ptr, &mut error) {
                    Ok(())
                }
                else {
                    Err(::Error::new(error))
                }
        }
    }
}

impl Drop for Notification {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.ptr as *mut libc::c_void);
        }
    }
}
