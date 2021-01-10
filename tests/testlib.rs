use error_remark::ResultExt;

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

#[test]
fn debug_fmt_long() {
    #[derive(Debug)]
    struct MyError {
        foo: i32,
        bar: i32,
    }

    let result = Result::<(), MyError>::Err(MyError { foo: 256, bar: 512 })
        .err_remark("spaghetti");
    let debug = format!("{:#x?}", result);
    assert_eq!(debug, r#"Err(
    {
        "spaghetti": MyError {
            foo: 0x100,
            bar: 0x200,
        },
    },
)"#);
}

#[test]
fn debug_fmt_short() {
    #[derive(Debug)]
    struct MyError {
        foo: i32,
        bar: i32,
    }

    let result = Result::<(), MyError>::Err(MyError { foo: 256, bar: 512 })
        .err_remark("spaghetti");
    let debug = format!("{:x?}", result);
    assert_eq!(debug, r#"Err({"spaghetti": MyError { foo: 100, bar: 200 }})"#);
}
