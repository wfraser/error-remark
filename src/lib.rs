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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stacking_messages() {
        let first = Result::<(), _>::Err("first inner").err_remark("first remark").unwrap_err();
        assert_eq!(first.to_string(), "first remark: first inner");

        let second = Result::<(), _>::Err(first).err_remark("second remark").unwrap_err();
        assert_eq!(second.to_string(), "second remark: first remark: first inner");
    }

    #[test]
    fn context_values() {
        let err = Result::<(), _>::Err("inner error")
            .err_remark_vars("context msg", &[&"static str", &77, &String::from("owned string too")])
            .unwrap_err();
        assert_eq!(
            err.to_string(),
            "context msg (\"static str\", 77, \"owned string too\"): inner error"
        );
    }

    #[test]
    fn deref_equality() {
        let result = Result::<(), i32>::Err(777).err_remark("hey there").unwrap_err();
        // Remark doesn't implement PartialEq or Eq, but you can deref it to get the inner.
        assert_eq!(*result, 777);
    }
}
