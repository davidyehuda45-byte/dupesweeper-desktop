pub mod categories;
pub mod executor;
pub mod recycle_bin;
pub mod scanner;

pub use categories::{
    CategoryScanResult, CleanupCategoryDef, CleanupCategoryId, CleanupItem, SafetyLevel,
    CLEANUP_CATEGORIES,
};
pub use executor::{CleanupExecutionReport, CleanupExecutor, CleanupProgressEvent};
pub use scanner::CleanupScanner;

