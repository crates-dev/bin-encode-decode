pub(crate) mod r#const;
pub(crate) mod error;
pub(crate) mod r#impl;
pub(crate) mod r#struct;

pub use {error::*, r#struct::*};

#[cfg(test)]
mod test;
