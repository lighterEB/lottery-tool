use crate::model::Number;

// 生成福彩3D/体彩排列三号码的函数
pub fn generate_numbers() -> Vec<Number> {
    let mut numbers = Vec::new();

    for h in 0..10 {
        for t in 0..10 {
            for u in 0..10 {
                // 创建新的 Number 实例
                let number = Number::new(h, t, u);
                // 将生成的号码添加到列表中
                numbers.push(number);
            }
        }
    }
    numbers
}