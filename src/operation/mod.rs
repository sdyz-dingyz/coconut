/// 字节操作模块
pub mod byte_operation;
/// Base64操作模块
pub mod base64_operation;

/// 重导出Operation Trait
pub use byte_operation::ByteOperation;
pub use base64_operation::Base64Operation;
