pub use xtask_cmdwrap_macro::cmd as command;

// TODO: guard behind feature flags
#[cfg(feature = "make")]
pub mod make;
