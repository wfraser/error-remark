use crate::Remark;
use std::fmt::{Debug, Display, Write};

/// An extension trait adding remark methods to `Result`.
pub trait ResultExt<T, E> {
    /// Add a remark if the result is an Err.
    fn remark(self, msg: &'static str) -> Result<T, Remark<E>>;

    /// Add a remark if the result is an Err, with some extra variables for additional context.
    /// Variables' Debug representations will be added to the remark message.
    fn remark_vars(self, msg: &'static str, stuff: &[&dyn Debug]) -> Result<T, Remark<E>>;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
    where E: Display + Debug + 'static
{
    fn remark(self, msg: &'static str) -> Result<T, Remark<E>> {
        self.map_err(|e| Remark::new_str(e, msg))
    }

    fn remark_vars(self, msg: &'static str, stuff: &[&dyn Debug]) -> Result<T, Remark<E>> {
        self.map_err(|e| {
            let mut s = format!("{} (", msg);
            let mut things = stuff.iter().peekable();
            while let (Some(thing), next) = (things.next(), things.peek()) {
                write!(&mut s, "{:?}", thing).unwrap();
                if next.is_some() {
                    s += ", ";
                }
            }
            s += ")";
            Remark::new_string(e, s)
        })
    }
}
