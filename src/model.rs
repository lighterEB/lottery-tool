use serde::{Deserialize, Serialize};

// 定义 Number 结构体, 表示福彩3D/体彩排列三号码
// 包含百位、十位和个位的数字
// 使用 u8 类型来表示每一位数字，范围从 0 到 9
// 该结构体可以用于序列化和反序列化操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Number {
    pub hundreds: u8,
    pub tens: u8,
    pub units: u8,
}

impl Number {
    // 创建新的 Number 实例
    pub fn new(hundreds: u8, tens: u8, units: u8) -> Self {
        Number {
            hundreds,
            tens,
            units,
        }
    }

    // 计算号码和值
    pub fn sum(&self) -> u8 {
        self.hundreds + self.tens + self.units
    }
}

