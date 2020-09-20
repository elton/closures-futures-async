// Copyright 2020 Elton Zheng
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// 接受一个闭包作为函数的参数
fn receives_closure<F>(closure: F)
where
    F: Fn(i32) -> i32,
{
    let result = closure(1);
    println!("closure(1) => {}", result);
}

/// 返回一个闭包
fn returns_clousure() -> impl Fn(i32) -> i32 {
    |x| x + 4
}

/// 一个currry函数
fn currry<F>(f: F, x: i32) -> impl Fn(i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    move |y| f(x, y) // 如果不加入move，则会报 closure may outlive the current function, but it borrows `f`, which is owned by the current function
}

/// 泛型Curry函数
fn generic_curry<F, X, Y, Z>(f: F, x: X) -> impl Fn(Y) -> Z
where
    F: Fn(X, Y) -> Z,
    X: Copy,
{
    move |y| f(x, y)
}

pub fn run() {
    let y = 2;
    let add = |x| x + y;
    let result = add(1);
    println!("{}", result);

    receives_closure(add);

    let closure = returns_clousure();
    println!("closure(1) => {}", closure(1));

    let add = |x, y| x + y;
    let closure = currry(add, 5);
    println!("closure(1) => {}", closure(1));

    let two = 2;
    let add = |x, y| x + y + two;
    let closure = generic_curry(add, 4);
    receives_closure(closure);

    let concat = |s, t: &str| format!("{}{}", s, t);
    let closure = generic_curry(concat, "Hello, ");
    let result = closure("world!");
    println!("{}", result);
}
