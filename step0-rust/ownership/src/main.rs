fn main() {
    foo();
    iter_vec();
    tuple_struct();
    match_fn();
    let a1 = A::Some(B::Some(55));
    foo2(a1);
}

fn foo() {
    // Creates a Vec object.
    // Gives ownership of the Vec object to v1.
    let mut v1 = vec![1, 2, 3];
    v1.pop();
    v1.push(4);
    v1.push(5);
    v1.clear();
    // drop(v1);
    v1.push(6);

    println!("{:?}", v1);
    // At the end of the scope, v1 goes out of scope.
    // v1 still owns the Vec object, so it can be cleaned up.

    let v_ref = &v1; // 引用的生命周期会在最后一次被使用时结束,取决于控制流图,不等于其词法作用域
    assert_eq!(v1[0], v_ref[0]); // v_ref 在这段代码运行完后结束
    let v_new = v1; // 所以这里移交v1所有权不会报错
    println!("{:?}", v_new);

    let mut vector: Vec<i32> = vec![];
    let vector_ref: &mut Vec<i32> = &mut vector;
    push(vector_ref, 4);
    push(&mut vector, 5);
    println!(" vector:{:?}", vector,);
    // println!(" vector_ref:{:?}", vector_ref,);
    // println!("vector:{:?}, vector_ref:{:?}",vector, vector_ref);

    let mut a = 8;
    println!("{}", *(&mut a) + 4);

    let x: i32 = 12;
    let y = x;
    println!("x still works: {}, and so does y: {}", x, y);

    let x1: Vec<i32> = vec![1, 2, 3];
    let y1 = &x1;
    println!("x1 still works: {:?}, and so does y1: {:?}", x1, y1);
}

fn push(vec_ref: &mut Vec<i32>, x: i32) {
    vec_ref.push(x);
}

fn iter_vec() {
    let mut vs = vec![1, 23, 4, 4];

    for v in &vs {
        println!("I'm borrowing {v}.");
    }

    for v in &mut vs {
        *v += 1;
        println!("I'm mutably borrowing {v}.");
    }

    for v in &vs {
        println!("Not borrowing {v}.");
    }
    println!("later {:?}", vs);
}

fn tuple_struct() {
    #[derive(PartialEq)]
    struct Meters(i32);
    #[derive(PartialEq)]
    struct Yards(i32);

    let a = Meters(32);
    let b = Meters(32);

    assert!(a == b);
}

fn match_fn() {
    let x = 17;
    match x {
        // r is reference type
        ref r => println!("Of type &i32: {}", r),
    }

    let mut y = 5;
    match y {
        ref r if y == 5 => println!("{r}"),
        ref mut r => *r *= 5,
    }
    println!("update info {y}");

    let mut v = vec![1, 2, 3];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

#[derive(Debug)]
enum A {
    None,
    Some(B),
}
#[derive(Debug)]
enum B {
    None,
    Some(i32),
}

fn foo2(x: A) {
    match x {
        a @ A::None => println!("a is A::{:?}", a),
        ref a @ A::Some(B::None) => println!("a is A::{:?}", *a),
        /* 
        所有权在A 此时b1 可获取 B类型所有权 或 引用
        A::Some(b1 @ B::Some(_)) => println!("b1 is B::{:?}", b1),
        A::Some(ref b1 @ B::Some(_)) => println!("b1 is B::{:?}", b1),
        */
        /*
        ref d @ A::Some(ref b1 @ B::Some(_)) => println!("d is A::{:?}, b1 is B::{:?}", d, b1),

        */
        ref d @ A::Some(ref b1 @ B::Some(_)) => println!("d is A::{:?}, b1 is B::{:?}", d, b1),

        // ref c @ A::Some(ref b @ B::Some(_)) => println!("B is B::{:?}, A is C::{:?}", b, c),
    }
}
