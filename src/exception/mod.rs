/// 标准异常模块
pub mod cresult;
pub use cresult::{CError, CResult};

pub mod operation
{
    use super::*;
    pub mod byte_operation
    {
        use super::*;
        pub const VEC_LENGTH_SHOULD_BE_SAME : &CError = &CError::new(1001, "进行字节操作的两个数组长度需要相同");
    }

    pub mod base64_operation
    {
        use super::*;
        pub const CANNOT_CONVERT_BASE64_STRING_TO_BYTE_SEQ : &CError = &CError::new(1022, "无法将Base64字符串转化成字节序列");
    }
}
