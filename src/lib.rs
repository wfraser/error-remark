// Copyright (c) 2020 by William R. Fraser

#![deny(rust_2018_idioms)]

/// Do you hate calling a complex function, just to get an error like "operation not permitted"
/// that bubbles up from deep within it? What was the operation that caused it? What arguments were
/// passed? Who knows, because `std::io::Error` is essentially just an integer.
///
/// Remark allows you to attach a more meaningful note to errors, which will help in debugging if
/// anything goes wrong. You can include a `&'static str` to errors which adds almost no overhead
/// at all, while making your life much easier later. Or to be even more helpful, you can build a
/// message that includes more info, like the parameters passed to a system call.
///
/// Remark strives to be lightweight and only incur overhead in the error case, so you don't have
/// an excuse to not just sprinkle it anywhere. Because who knows where the error your future self
/// will be debugging will come from?

mod remark;
mod result_ext;

pub use remark::Remark;
pub use result_ext::ResultExt;
