pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 这里省略了方法的默认实现
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") }, 
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();// 之所以不要求v1_iter可变，是因为循环取得了v1_iter的所有权并在内部使得它可变了
    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1: Vec<i32> = vec![1, 2, 3];
    // v1.iter().map(|x| x + 1); // 这里不会有任何效果，因为map是惰性的，必须使用collect()来消费迭代器
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None); 
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // 注意：调用sum()之后，v1_iter就不能再使用了，因为sum()会获取v1_iter的所有权
    // println!("{:?}", v1_iter); // error[E0382]: borrow of moved value: `v1_iter`
}