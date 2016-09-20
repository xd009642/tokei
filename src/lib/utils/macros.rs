// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

macro_rules! opt_info {
    ($option:expr, $message:expr) => {
        match $option {
            Some(result) => result,
            None => {
                info!($message);
                continue;
            },
        }
    }
}

macro_rules! rs_info {
    ($result:expr, $message: expr) => {
        match $result {
            Ok(result) => result,
            Err(error) => {
                use std::error::Error;
                info!("{}", error.description());
                continue;
            },
        }
    }
}

macro_rules! opt_warn {
    ($option:expr, $message:expr) => {
        match $option {
            Some(result) => result,
            None => {
                warn!($message);
                continue;
            },
        }
    }
}

macro_rules! rs_warn {
    ($result:expr) => {
        match $result {
            Ok(result) => result,
            Err(error) => {
                use std::error::Error;
                warn!("{}", error.description());
                continue;
            },
        }
    }
}


macro_rules! debug {
    ($fmt:expr) => (if cfg!(debug_assertions) {println!($fmt)});
    ($fmt:expr, $($arg:tt)*) => (if cfg!(debug_assertions) {println!($fmt, $($arg)*)});
}
