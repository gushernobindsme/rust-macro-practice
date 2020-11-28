#![allow(unused_macros)]
/// 一番スタンダードなマクロ
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

/// item。アイテム（関数、構造体など）に対してマッチする。
macro_rules! match_item {
    ($i:item) => {
        println!("item: {}", stringify!($i))
    };
}

/// block。{ } で囲まれるブロックにマッチする。
macro_rules! match_block {
    ( $b:block ) => {
        println!("block: {}", stringify!($b))
    };
}

/// stmt。文にマッチする。
macro_rules! match_stmt {
    ( $s:stmt ) => {
        println!("stmt: {}", stringify!($s))
    };
}

/// pat。match などで使われるパターンにマッチする。
macro_rules! match_pat {
    ( $p:pat ) => {
        println!("pat: {}", stringify!($p))
    };
}

/// expr。式にマッチする。
macro_rules! match_expr {
    ( $e:expr ) => {
        println!("expr: {}", stringify!($e))
    };
}

/// ty。型にマッチする。
macro_rules! match_ty {
    ( $t:ty ) => {
        println!("ty: {}", stringify!($t))
    };
}

/// ident。識別子にマッチする。
macro_rules! match_ident {
    ( $i:ident ) => {
        println!("ident: {}", stringify!($i))
    };
}

/// path。パスにマッチする。
macro_rules! match_path {
    ( $p:path ) => {
        println!("path: {}", stringify!($p))
    };
}

/// tt。単一のトークン木にマッチする。
macro_rules! match_tt {
    ( $t:tt ) => {
        println!("tt: {}", stringify!($t))
    };
}

/// meta。アトリビュートの中身にマッチする。
macro_rules! match_meta {
    ( $m:meta ) => {
        println!("meta: {}", stringify!($m))
    };
}

/// マッチ条件は複数指定できる
macro_rules! foo {
    (x => $e:expr) => {
        println!("mode X: {}", $e)
    };
    (y => $e:expr) => {
        println!("mode Y: {}", $e)
    };
}

/// 繰り返し。
/// カンマ（,）で区切られた 0 個以上の式を受け取ることができる。
/// 受け取った式は配列にして返す。
macro_rules! array {
    ( $($x:expr),* ) => { [ $( $x ),* ] };
}

/// 繰り返し。
/// コロン（;）で区切られた 0 個以上の式を受け取ることができる。
/// 受け取った式は Vec にして返す。
macro_rules! myvec {
    ( $( $x:expr );* ) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}

/// 再帰呼び出し。こんな風に呼び出すと…
/// ```rust
/// write_html!(&mut out,
/// html[
///     head[title["Macros guide"]]
///     body[h1["Macros are the best!"]]
/// ]);
/// ```
/// こんな風に HTML タグを生成して返す。
/// ```
/// <html><head><title>Macros guide</title></head>
/// <body><h1>Macros are the best!</h1></body></html>
/// ```
macro_rules! write_html {
    ($w:expr, ) => {
        ()
    };
    ($w:expr, $e:tt) => {
        write!($w, "{}", $e);
    };
    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag));
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag));
        write_html!($w, $($rest)*);
    }};
}

/// 入れ子の繰り返し。
macro_rules! o_O {
    ( $( $x:expr; [ $( $y:expr ),* ]);+ ) => {
        &[ $($( $x + $y ),*),+ ]
    };
}

fn main() {
    // マクロの基本
    say_hello!();

    // item
    match_item!(
        fn do_something() {}
    );
    match_item!(
        mod hoge {}
    );
    match_item!(
        struct Hoge {
            calue: i8,
            text: String,
        }
    );

    // block
    match_block!({});
    match_block!({ 10 });
    match_block!({
        let x = 10;
        println!({}, x);
    });

    // stmt
    match_stmt!(let a = 10);
    match_stmt!(10);
    match_stmt!(println!("{}", 5));
    match_stmt!({ 10 });
    match_stmt!({
        let x = 10;
        println!({}, x)
    });
    match_stmt!(
        struct Hoge {
            value: i8,
            text: String,
        }
    );
    match_stmt!(
        macro_rules! test {
            () => {};
        }
    );
    match_stmt!(
        fn do_something() {}
    );
    match_stmt!(
        mod hoge {}
    );
    match_stmt!(MyType);
    match_stmt!(std::str);

    // pat
    match_pat!(1);
    match_pat!(_);
    match_pat!(2...9);
    match_pat!(e @ 1 ... 5);
    match_pat!(None);
    match_pat!(Some(x));

    // expr
    match_expr!(10);
    match_expr!(10 + 2 * 100);
    match_expr!({
        let result = do_something();
        report_result(result);
        result
    });
    match_expr!(if initialized { start() } else { report_error() });
    match_expr!(if initialized {
        start()
    });

    // ty
    match_ty!(i32);
    match_ty!(String);
    match_ty!(&[i32]);
    match_ty!(UnknownType); // 未知の型でも大丈夫
    match_ty!(html); // 小文字始まりの未知の型でも大丈夫
    match_ty!(std::str); // パス付きの型でも大丈夫

    // ident
    match_ident!(i32);
    match_ident!(String);
    match_ident!(UnknownType);
    match_ident!(match);
    match_ident!(if);
    match_ident!(else);

    // path
    match_path!(i32);
    match_path!(::main);
    match_path!(std::str);

    // tt
    match_tt!({ println!("Hello world!") });
    match_tt!(i32);
    match_tt!(_);
    match_tt!(10);
    match_tt!({ 10 });
    match_tt!({
        let x = 10;
        println!({}, x);
    });
    match_tt!({ name:[ tag1(), tag2() ] });
    match_tt!({
        values: [
            1, 2, 3, 4
        ],
        kv: {
            "prefecture" => "Aichi",
            "city" => "Nagoya",
        }
    });

    // パラメータを指定することもできる
    foo!(x => 1);
    foo!(y => 3);

    // 繰り返し
    let ar = array!(1, 2, 3, 4);
    println!("{:?}", ar);
    let v = myvec![1; 2; 3; 4];
    println!("{:?}", v);
    let o = o_O!(10; [1, 2, 3]; 20; [4, 5, 6]);
    println!("{:?}", o);
}
