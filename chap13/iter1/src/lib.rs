struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {// Iterator是一个trait，它定义了迭代器模式（iterator pattern）的行为，这里我们只需要提供next方法的实现
    type Item = u32;// type Item;是trait的关联类型（associated type），它定义了迭代器正在迭代的值的类型。在这里，我们希望迭代u32值，所以将Item类型指定为u32。关联类型的主要好处是实现trait时可以定义使用的具体类型，而不是在trait中使用泛型参数。这对于迭代器来说是必要的，因为迭代器实现者会在next方法中使用这个类型，而调用next方法的代码不需要知道这个类型是什么。
    
    fn next(&mut self) -> Option<Self::Item> {// next方法返回Option<Self::Item>，这是因为迭代器可能会结束。Option是一个拥有Some(T)或None值的枚举，这里用来表明迭代器是否有多余的元素可以继续迭代。next方法是迭代器实现者需要提供的唯一方法，next会获取self的所有权，这样迭代器就可以在每次调用之间保持迭代状态，例如，Counter迭代器需要知道它在计数到多少了。next方法通常会从迭代器指向的序列中返回一个元素，接着迭代器就会记录它的位置以便下次调用。不过，next也可以在没有元素的时候返回None并结束迭代。在这个例子中，当count等于5时，next返回None，这时迭代器就结束了。
        self.count += 1;
        
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {// 调用next方法来消费Counter迭代器
    let mut counter = Counter::new();
    
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {// 使用其他Iterator trait方法
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))// zip方法将两个迭代器合并到一个元组中。当其中一个迭代器返回None时，zip也会返回None
                                 .map(|(a, b)| a * b)// map方法会创建一个新迭代器，其元素是原迭代器中的元素调用闭包后的返回值
                                 .filter(|x| x % 3 == 0)// filter方法创建一个只包含原迭代器中满足指定条件的元素的新迭代器
                                 .sum();// sum方法获取迭代器的所有权并反复调用next来遍历迭代器，因此sum调用之后原迭代器不能再被使用了
    assert_eq!(18, sum);
}