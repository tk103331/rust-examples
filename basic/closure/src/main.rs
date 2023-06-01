fn call_once(c: impl FnOnce()) {
    c();
}
fn call_mut(c: &mut impl FnMut()) {
    c();
}
fn call_fn(c: impl Fn()) {
    c();
}
fn main() {
    // 1、闭包use_closure1只实现了FnOnce Trait，只能被调用一次
    let s = "Hello".to_string();
    let use_closure1 = move || {
        let s1 = s;
        println!("s1 = {:?}", s1);
    };
    use_closure1(); // 此行打印“s1 = "Hello"”
                    // println!("s = {:?}", s); // 编译错误：因为s所有权已经被移动闭包中use_closure1中
                    // use_closure1();  // 编译错误：多次调用use_closure1出错
    let s = "Hello".to_string();
    let use_closure11 = move || {
        let s1 = s;
        println!("s1 = {:?}", s1);
    };
    call_once(use_closure11);

    // 2、闭包use_closure2只实现了FnOnce Trait和FnMut Trait
    let mut s = "Hello".to_string();
    let mut use_closure2 = || {
        s.push_str(", world!");
        println!("s = {:?}", s);
    };
    use_closure2(); // 此行打印“s = "Hello, world!"”
    use_closure2(); // 可以多次调用，此行打印“s = "Hello, world!, world!"”
    call_mut(&mut use_closure2);
    call_mut(&mut use_closure2);
    call_once(use_closure2);


    // 3、闭包use_closure3实现了FnOnce Trait、FnMut Trait和Fn Trait
    let s = "Hello".to_string();
    let mut use_closure3 = || {
        println!("s = {:?}", s);
    };
    use_closure3(); // 此行打印“s = "Hello"”
    use_closure3(); // 可以多次调用，此行打印“s = "Hello!"”
    call_mut(&mut use_closure3);
    call_fn(use_closure3);
    
    call_once(use_closure3);
}