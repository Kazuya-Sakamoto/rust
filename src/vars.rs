pub mod sub_a;
pub mod sub_b;

/**
 * アプリケーションのメモリ
 * Heap:    可変長データを格納する
 * Stack:   サイズが決まった変数や配列などを格納する
 * Static:  const, 文字リテラルの実体を格納する
 * Text:    Code
 */

/**
 * Last in First Out
 * Push: スタックにデータを追加する
 * Pop:  スタックからデータを取り出す
 */
const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();

    // * デフォルトimuutable
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // * _変数名で未使用変数を抑制
    let _i1 = 3;
    let _f1 = 0.1;

    // * ポインタ
    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    //* 8バイトずつずれる
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    // * シャドーイング
    let y = 5;
    println!("Stack value of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack value of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack value of y is: {:p}", &y);
    println!("The value of y is: {}", y);
    {
        let y = 0;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);

    // * タプル
    let t1: (i32, f64, &str) = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    // 参照外し
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    // * 配列
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    let s1 = "helloこんにちは挨拶";
    let s2 = "hello";
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    // * ヒープに格納される
    println!("Stack memory address of s1 is: {:p}", &s1.as_ptr());
    println!("Stack memory address of s2 is: {:p}", &s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());

    // * 文字列スライス
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    println!("Stack memory address of s1 is: {:p}", &s1.as_ptr());
    println!("Stack memory address of s2 is: {:p}", &s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2)
}
