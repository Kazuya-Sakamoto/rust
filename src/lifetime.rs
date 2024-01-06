
pub fn run() {
    let st1 = String::from("xx");
    let st2 = String::from("yy");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    let st3 = String::from("xxx");
    let res2; // res2 = get_longest(&st3, &st4); のところで型が逆算される
    {
        let st4 = String::from("yyy"); //st3よりライフタイムが短い
        res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    }
    // st4のライフタイムが終わっているのでエラーになる
    // println!("{}", res2); 
}

fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// fn dummy1<'a>() -> &'a str {
//     let s = String::from("demo");
//     &s
// }
// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     // 実体のxがスコープを抜けるのでエラーになる（dummy1と同じ）
//     // &x
// }

// エラーにならない
fn dummy3() -> String {
    let s = String::from("demo");
    s
}