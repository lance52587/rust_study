use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    let teams = vec![String::from("Blue"),String::from("Yellow")];
    let initial_scores = vec![10,50];

    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();//使用zip方法创建一个元组的vector，然后collect方法将元组vector转换成HashMap，collect方法的类型注解HashMap<_,_>是必要的，因为可能collect很多不同的数据结构，如果不注解，Rust就无法知道你需要的是哪种数据结构，而且也无法推断出键和值的类型，不过对于键和值的类型，可以使用HashMap<_,_>来代替，这样就可以根据collect方法使用的数据来推断出HashMap的类型了。

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name,field_value);//field_name和field_value的所有权都会被移动到hashmap中，所以在这里不能再使用它们了

    //println!("field_name = {}",field_name);//error[E0382]: borrow of moved value: `field_name`

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key,value) in &scores {
        println!("{}:{}",key,value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    println!("scores = {:?}",scores);
    scores.insert(String::from("Blue"),25);//如果键不存在，则插入，如果存在，则更新
    println!("scores = {:?}",scores);

    scores.entry(String::from("Yellow")).or_insert(50);//entry方法获取键对应的值，如果不存在，则插入参数指定的值，如果存在，则返回对应的值
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores = {:?}",scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;      
    }

    println!("map = {:?}",map);
}
