use crate::exception::CResult;
// 导入所需异常
use crate::exception::operation::byte_operation::*;

/// ByteOperation Trait用于对两个集合进行字节层面的操作
pub trait ByteOperation
{
    /// 对两个字节序列进行与操作
    fn and(&mut self, bv : &Self) -> CResult<()>;

    /// 对两个字节序列进行或操作
    fn or(&mut self, bv : &Self) -> CResult<()>;

    /// 对两个字节序列进行异或操作
    fn xor(&mut self, bv : &Self) -> CResult<()>;

    /// 判断两个字节序列是否相等
    fn equals(&self, b : &Self) -> bool;

}

/// ByteVec是一个字节序列，重定义自Vec<u8>
pub type ByteVec = Vec<u8>;

/// 随机生成一个字节序列
pub fn rand_byte_vec(len : usize) -> ByteVec
{
    use rand::{thread_rng, Rng};

    let mut arr = ByteVec::new();
    arr.resize(len, 0u8);
    thread_rng().fill(&mut arr[..]);
    arr
}

impl ByteOperation for ByteVec
{
    fn and(&mut self, bv : &Self) -> CResult<()> 
    {
        if self.len() != bv.len()
        {
            return VEC_LENGTH_SHOULD_BE_SAME.to_cresult();
        }
        let length = self.len();
        for i in 0..length
        {
            self[i] = self[i] & bv[i];
        }
        Ok(())
    }

    fn or(&mut self, bv : &Self) -> CResult<()> 
    {
        if self.len() != bv.len()
        {
            return VEC_LENGTH_SHOULD_BE_SAME.to_cresult();
        }
        let length = self.len();
        for i in 0..length
        {
            self[i] = self[i] | bv[i];
        }
        Ok(())
    }

    fn xor(&mut self, bv : &Self) -> CResult<()> 
    {
        if self.len() != bv.len()
        {
            return VEC_LENGTH_SHOULD_BE_SAME.to_cresult();
        }
        let length = self.len();
        for i in 0..length
        {
            self[i] = self[i] ^ bv[i];
        }
        Ok(())
    }

    fn equals(&self, b : &Self) -> bool
    {
        if self.len() != b.len()
        {
            return false;
        }
        let length = self.len();
        for i in 0..length
        {
            if self[i] != b[i]
            {
                return false;
            }
        }
        return true;
    }
    
}

impl ByteOperation for [u8]
{
    fn and(&mut self, bv : &Self) -> CResult<()> 
    {
        if self.len() != bv.len()
        {
            return VEC_LENGTH_SHOULD_BE_SAME.to_cresult();
        }
        let length = self.len();
        for i in 0..length
        {
            self[i] = self[i] & bv[i];
        }
        Ok(())
    }

    fn or(&mut self, bv : &Self) -> CResult<()> 
    {
        if self.len() != bv.len()
        {
            return VEC_LENGTH_SHOULD_BE_SAME.to_cresult();
        }
        let length = self.len();
        for i in 0..length
        {
            self[i] = self[i] | bv[i];
        }
        Ok(())
    }

    fn xor(&mut self, bv : &Self) -> CResult<()> 
    {
        if self.len() != bv.len()
        {
            return VEC_LENGTH_SHOULD_BE_SAME.to_cresult();
        }
        let length = self.len();
        for i in 0..length
        {
            self[i] = self[i] ^ bv[i];
        }
        Ok(())
    }

    fn equals(&self, b : &Self) -> bool
    {
        if self.len() != b.len()
        {
            return false;
        }
        let length = self.len();
        for i in 0..length
        {
            if self[i] != b[i]
            {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_byte_vec()
    {
        let a = vec![0xff, 0xff];
        let b = vec![0x66, 0x71];
        let mut a_and_b = a.clone();
        a_and_b.and(&b).unwrap();
        let mut a_or_b = a.clone();
        a_or_b.or(&b).unwrap();
        let mut a_xor_b = a.clone();
        a_xor_b.xor(&b).unwrap();
        println!("a and b: {:?}", a_and_b);
        println!("a or b: {:?}", a_or_b);
        println!("a xor b: {:?}", a_xor_b);
        println!("a equals a? = {:?}", a.equals(&a));
        println!("a equals b? = {:?}", a.equals(&b));
    }

    #[test]
    fn test_u8_vec()
    {
        let a : [u8; 3] = [0xff, 0xff, 0x96];
        let b : [u8; 3] = [0x66, 0x71, 0x86];
        let mut a_and_b = a;
        a_and_b.and(&b).unwrap();
        let mut a_or_b = a;
        a_or_b.or(&b).unwrap();
        let mut a_xor_b = a;
        a_xor_b.xor(&b).unwrap();
        println!("a and b: {:?}", a_and_b);
        println!("a or b: {:?}", a_or_b);
        println!("a xor b: {:?}", a_xor_b);
        println!("a equals a? = {:?}", a.equals(&a));
        println!("a equals b? = {:?}", a.equals(&b));
    }

    #[test]
    fn test_rand()
    {
        let a = rand_byte_vec(16);
        println!("a: {:?}", a);
    }
}
