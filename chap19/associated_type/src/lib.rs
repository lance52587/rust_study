// impl Iterator for Counter{
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         // --略
//     }
// }

use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {//指定impl Add<Meters>来设置RHS类型参数的值，而没有使用默认的Self
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}