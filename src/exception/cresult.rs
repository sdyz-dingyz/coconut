use std::borrow::ToOwned;

/// coconut库下的标准错误信息格式
#[derive(Debug)]
pub struct CError
{
    // 错误码
    m_code : u32,
    // 错误信息
    m_message : &'static str,
    // 其他补充信息
    m_other_info : Option<String>
}

impl CError 
{
    /// 创建一个CError
    pub const fn new(code : u32, message : &'static str) -> CError
    {
        CError 
        {
            m_code: code, 
            m_message: message, 
            m_other_info: None
        }
    }

    /// 添加附加信息
    pub fn add_other_info(&self, other_info : String) -> CError
    {
        CError
        {
            m_code : self.m_code,
            m_message : self.m_message,
            m_other_info : Some(other_info)
        }
    }

    /// 转换到Result
    pub fn to_cresult<T>(&self) -> CResult<T>
    {
        Err(self.to_owned())
    }

    /// 带other_info转换到Result
    pub fn to_cresult_with_other_info<T>(&self, other_info : String) -> CResult<T>
    {
        let e = self.add_other_info(other_info);
        Err(e)
    }
}

impl ToOwned for CError
{
    type Owned = CError;

    fn to_owned(&self) -> Self::Owned
    {
        CError
        {
            m_code : self.m_code,
            m_message : self.m_message,
            m_other_info : 
            {
                if let Some(ref info) = self.m_other_info 
                {
                    Some(info.clone())
                } 
                else 
                {
                    None
                }
            }
        }
    }
}

/// CResult是coconut库中的标准结果类型
pub type CResult<T> = Result<T, CError>;

#[cfg(test)]
mod tests 
{
    use super::*;
    /// 用于测试异常的异常，没有实际意义
    const EXCEPTION_TEST : &CError = &CError::new(0xffffffff, "测试：异常测试");

    #[test]
    fn test_exception() 
    {
        // Ok测试
        let e_a = EXCEPTION_TEST.to_owned();
        let r_a : Result<i32, i16> = Ok(2);
        r_a.or(e_a.to_cresult()).unwrap();
        // Err测试 + other_info测试
        let r_b : Result<i32, i16> = Err(3);
        r_b.or(EXCEPTION_TEST.to_cresult_with_other_info("补充".into())).unwrap();
    }
}