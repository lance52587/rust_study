use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T>{
        Cacher{
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32{
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32){
    // let expensive_result = simulated_expensive_calculation(intensity);
    // let expensive_closure = |num| {
    // let expensive_closure = |num: u32| -> u32 {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num+1
    // };
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",// 今天做{}个俯卧撑！
            // simulated_expensive_calculation(intensity)
            // expensive_result
            // expensive_closure(intensity)
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",// 接下来做{}个仰卧起坐！
            // simulated_expensive_calculation(intensity)
            // expensive_result
            // expensive_closure(intensity)
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
            // 今天休息一下！记得补充水分！
        } else {
            println!(
                "Today, run for {} minutes!",// 今天跑步{}分钟！
                // simulated_expensive_calculation(intensity)
                // expensive_result
                // expensive_closure(intensity)
                expensive_closure.value(intensity)
            );
        }
    }
}

fn main(){
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);// error[E0308]: mismatched types

    let x = 4;
    let equal_to_x = |z| z == x;
    // fn equal_to_x(z: i32) -> bool { z == x; }// error[E0434]: can't capture dynamic environment in a fn item
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1,2,3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);// error[E0382]: borrow of moved value: `x`
    let y = vec![1,2,3];
    assert!(equal_to_x(y));
}

#[test]
fn call_with_different_values(){
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    assert_eq!(v1, 1);
    let v2 = c.value(2);
    // assert_eq!(v2, 2);// assertion failed: `(left == right)` left: `1`, right: `2`
}