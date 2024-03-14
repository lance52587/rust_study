fn main() {
    let buffer: &mut [i32; 12];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    for i in 12..buffer.len(){
        let prediction = coefficients.iter()
        .zip(&buffer[i-12..i])// 12个系数和12个历史样本
        .map(|(&c,&s)| c * s as i64)// 乘法
        .sum::<i64>() >> qlp_shift;// 累加和右移
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;// 预测值加上残差
    }
}
