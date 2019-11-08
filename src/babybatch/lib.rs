// Module declarations.
pub mod comm;
pub mod structure;
pub mod util;

/// Re-exports.
pub use util::command_prelude;

pub use structure::resource::Resource;
pub use structure::thread_pool::ThreadPool;
pub use structure::BabyBatchResponse;
pub use util::errors::{BabyBatchError, Result};
