use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream{
}

#[macro_export]
macro_rules! vec {// 被引入作用域后可用。缺少了这个标注的宏则不能被引入作用域。
    ( $( $x:expr ),* ) => {// 模式匹配：此处$表示一个变量，x表示变量名，expr表示表达式，*表示0个或多个。
        // $()中的$x:expr可以匹配任意的Rust表达式，并将其命名为$x。
        // $()之后的逗号意味着一个可能的字面逗号分隔符会出现在捕获代码的后面，而逗号后的*则意味着这个模式能够匹配零个或多个*之前的东西。
        // 当我们使用指令vec![1, 2, 3];调用这个宏时，$x模式会分别匹配3个表达式：1、2及3。
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            // 会被展开为：
            // temp_vec.push(1);
            // temp_vec.push(2);
            // temp_vec.push(3);
            temp_vec
        }
    };
}