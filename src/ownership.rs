pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    // 所有権のムーブ。s1には所有権がないのでエラー
    println!("{}", s2);

    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);

    // ヒープ内のデータはコピーされる
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{} {}", s3, s4);
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap memory address of hello: {:?}", s3.as_ptr());
    println!("Heap memory address of hello: {:?}", s4.as_ptr());

    // 関数呼び出しで所有権がムーブされる
    let s5: String = String::from("hello");
    println!("Stack address of s5 is: {:p}", &s5);
    println!("Heap memory address of s5: {:?}", s5.as_ptr());
    println!("Len of s5 is: {}", s5.len());
    println!("Capacity of s5 is: {}", s5.capacity());
    takes_ownership(s5);
    // 関数引数で渡すと所有権がムーブされているのでエラー
    // println!("{}", s5)

    let s6 = String::from("hello");
    println!("Stack address of s6 is: {:p}", &s6);
    println!("Heap memory address of s6: {:?}", s6.as_ptr());
    println!("Len of s6 is: {}", s6.len());
    let s7 = takes_giveback_ownership(s6);
    println!("Stack address of s7 is: {:p}", &s7);
    println!("Heap memory address of s7: {:?}", s7.as_ptr());
    println!("Len of s7 is: {}", s7.len());

    let s8 = String::from("hello");
    let len = caluculate_length(&s8);
    println!("The Length of '{}' is {}.", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // 1つでも可変参照があるとエラーになる
    // let r2 = &mut s10;

    let mut s11: String = String::from("hello");
    let r1 = &mut s11;
    println!("{}", r1);
    println!("{}", s11);

    let mut s12: String = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("hello_updated");
    println!("{}", s12);
}

fn takes_ownership(s: String) {
    println!("Stack address of s is: {:p}", &s);
    println!("Heap memory address of s: {:?}", s.as_ptr());
    println!("Len of s is: {}", s.len());
    println!("Capacity of s is: {}", s.capacity());
    println!("{}", s);
}

fn takes_giveback_ownership(s: String) -> String {
    // println!("Stack address of s is: {:p}", &s);
    // println!("Heap memory address of s: {:?}", s.as_ptr());
    // println!("Len of s is: {}", s.len());
    // println!("Capacity of s is: {}", s.capacity());
    // println!("{}", s);
    s
}

fn caluculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}