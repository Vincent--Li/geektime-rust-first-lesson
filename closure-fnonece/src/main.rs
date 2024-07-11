fn main() {
    let name = String::from("VIncent");

    // 这个闭包会clone内部数据返回, 所以不是FnOnce
    let c1 = move |gretting: String| (gretting, name.clone());

    // 所以c1 可以被调用多次
    println!("c1 call once: {:?}", c1("qiao".to_string()));
    println!("c1 call twice: {:?}", c1("bonjour".to_string()));

    // 一旦被当成FnOnce, 就不能再被调用了
    // println!("result: {:?}", call_once("hi".into(), c1));

    // 无法再次调用
    let _ = c1("hi".to_string());

    // Fn 也可以被当做FnOnce调用, 只要接口一致就可以
    println!("result: {:?}", call_once("hola".into(), not_closure));

}

fn call_once(arg: String, f: impl FnOnce(String) -> (String, String)) -> (String, String) {
    f(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}
