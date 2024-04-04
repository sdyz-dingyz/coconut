use crate::exception::CResult;
use base64::{Engine as _, engine::general_purpose};
use crate::operation::byte_operation::ByteVec;
use crate::exception::operation::base64_operation::*;

/// Base64编码操作
pub trait Base64Operation
{
    /// 将字节序列转化成base64字符串
    fn to_base64_string(&self) -> String;

    /// 将base64字符串转换成字节序列
    fn from_base64_string(b64str : &str) -> CResult<Self>
        where Self : Sized;
}

impl Base64Operation for ByteVec
{
    fn to_base64_string(&self) -> String
    {
        let mut buf = String::new();
        general_purpose::STANDARD
            .encode_string(self, &mut buf);
        buf
    }

    fn from_base64_string(b64str : &str) -> CResult<Self>
    {
        let mut buf = Vec::<u8>::new();
        general_purpose::STANDARD
            .decode_vec(b64str, &mut buf)
            .or(CANNOT_CONVERT_BASE64_STRING_TO_BYTE_SEQ.to_cresult())?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    
    #[test]
    fn test_base64()
    {
        let a = ByteVec::from_base64_string("Vn8=").unwrap();
        assert_eq!("Vn8=".to_string(), a.to_base64_string());
    }
}