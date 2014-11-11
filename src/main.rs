extern crate notify;
extern crate getopts;
use std::os;
use std::time::duration::Duration;
use std::from_str::from_str;
use getopts::{optopt,optflag,getopts};
use std::string::String;

fn main() {
    let args = os::args();
    let program = args[0].clone();

    let opts = [
        optopt("u", "urgency", "Specifies urgency level (low, normal, critical)", "LEVEL"),
        optopt("t", "expire-time", "Specifies the timeout at which to expire the notification", "TIME"),
        optopt("i", "icon", "Specifies the icon filename or stock icon to display.", "ICON"),
        optopt("c", "category", "Specifies the notification category", "TYPE"),
        optflag("h", "help", "Print this help")
    ];

    let blank = String::from_str("");

    let mut matches = getopts(args.tail(), opts).unwrap();
    if matches.opt_present("h") || (matches.free.len() < 1) {
        let brief = format!("Usage: {} [options] <summary> [description]", program);
        println!("{}", getopts::usage(brief.as_slice(), opts));
        return;
    }

    let summary = matches.free.remove(0).unwrap();
    let description = matches.free.remove(0).unwrap_or(blank.clone());
    let icon = matches.opt_str("i").unwrap_or(blank.clone());

    notify::init(args[0].as_slice());
    let notification = notify::Notification::new(summary.as_slice(), description.as_slice(), icon.as_slice());

    match matches.opt_str("c") {
        Some(category) => notification.set_category(category.as_slice()),
        None => ()
    }
    match matches.opt_str("t") {
        Some(expires) => {
            let expires: i64 = from_str(expires.as_slice()).unwrap();
            let expires = Duration::milliseconds(expires);
            notification.set_timeout(expires);
        },
        None => ()
    }

    notification.show();
    notify::uninit();
}
