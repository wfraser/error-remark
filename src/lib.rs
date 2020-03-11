// Copyright (c) 2020 by William R. Fraser

#![deny(rust_2018_idioms)]

mod remark;
mod result_ext;

pub use remark::Remark;
pub use result_ext::ResultExt;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stacking_messages() {
        let first = Result::<(), _>::Err("first inner").remark("first remark").unwrap_err();
        assert_eq!(first.to_string(), "first remark: first inner");

        let second = Result::<(), _>::Err(first).remark("second remark").unwrap_err();
        assert_eq!(second.to_string(), "second remark: first remark: first inner");
    }

    #[test]
    fn context_values() {
        let err = Result::<(), _>::Err("inner error")
            .remark_vars("context msg", &[&"static str", &77, &format!("owned string too")])
            .unwrap_err();
        assert_eq!(
            err.to_string(),
            "context msg (\"static str\", 77, \"owned string too\"): inner error"
        );
    }

    #[test]
    fn deref_equality() {
        let result = Result::<(), i32>::Err(777).remark("hey there").unwrap_err();
        // Remark doesn't implement PartialEq or Eq, but you can deref it to get the inner.
        assert_eq!(*result, 777);
    }
}
