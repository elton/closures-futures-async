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
use futures::future;
use futures::future::FutureExt;
use log::debug;
use std::convert::Infallible;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::delay_for;

fn returns_impl_future_i32() -> impl Future<Output = i32> {
    future::ready(42)
}

fn returns_dyn_future_i32() -> Pin<Box<dyn Future<Output = i32>>> {
    if rand::random() {
        Box::pin(future::ready(42))
    } else {
        Box::pin(future::lazy(|_| 1337))
    }
}

fn returns_future_result() -> impl Future<Output = Result<i32, impl Error>> {
    future::ok::<_, Infallible>(42) // the _ is inferred from the parameter type
}

fn returns_future_result_dyn_error() -> impl Future<Output = Result<i32, Box<dyn Error>>> {
    future::ok(42)
}

fn returns_delayed_future() -> impl Future<Output = i32> {
    delay_for(Duration::from_millis(500)).then(|_| futures::future::ready(42))
}

fn wait_a_sec<F, O>(f: F) -> impl Future<Output = O>
where
    F: Future<Output = O>,
{
    let delay = Duration::from_millis(1000);
    delay_for(delay).then(|_| f)
}

fn returns_future_chain() -> impl Future<Output = ()> {
    future::lazy(|_| debug!("in returns_future_chain()"))
        .then(|_| {
            debug!("in first then");
            future::ready("Hello from rt.block_on()")
        })
        .inspect(|result| debug!("future::ready() -> {}", result))
        .then(|_| returns_impl_future_i32())
        .inspect(|result| debug!("returns_impl_future_i32() -> {}", result))
        .then(|_| returns_dyn_future_i32())
        .inspect(|result| debug!("returns_dyn_future_i32() -> {}", result))
        .then(|_| returns_future_result())
        .map(|result| result.unwrap())
        .inspect(|result| debug!("returns_future_result().unwrap() -> {}", result))
        .then(|_| returns_future_result_dyn_error())
        .map(|result| result.unwrap())
        .inspect(|result| debug!("returns_future_result_dyn_error().unwrap() -> {}", result))
        .then(|_| returns_delayed_future())
        .inspect(|result| debug!("returns_delayed_future() -> {}", result))
        .then(|_| wait_a_sec(future::ready(42)))
        .inspect(|result| debug!("wait_a_sec(future::ready(42)) -> {}", result))
        .then(|_| {
            debug!("in last then");
            future::ready(())
        })
}

pub async fn run() {
    // 初始化 Tokio runtime
    // both rt.enter() and rt.block_on() run on the main() thread (thread id(1))，他们是在主线程执行的，所以是有先后顺序的
    // Both tokio::spawn() and rt.spawn() may schedule the given Future on a different thread.，他们在不同线程中执行，所以顺序是随机的，甚至没有执行
    // let mut rt = tokio::runtime::Builder::new()
    //     .threaded_scheduler()
    //     .core_threads(4) // 启用4个线程
    //     .on_thread_start(|| debug!("on_thread_start()"))
    //     .build()
    //     .unwrap();
    // // enter传入一个FnOnce闭包
    // rt.enter(|| {
    //     debug!("in rt.enter()");
    //     tokio::spawn(future::lazy(|_| debug!("in tokio::spawn")));
    // });
    // rt.spawn(future::lazy(|_| debug!("in rt.spawn()")));
    // rt.block_on(future::lazy(|_| debug!("in rt.block_on()")));

    // 04:30:00 [DEBUG] (1) in rt.enter()
    // 04:30:00 [DEBUG] (1) in rt.block_on()
    // 04:30:00 [DEBUG] (2) on_thread_start()
    // 04:30:00 [DEBUG] (4) on_thread_start()
    // 04:30:00 [DEBUG] (4) in rt.spawn()
    // 04:30:00 [DEBUG] (5) on_thread_start()
    // 04:30:00 [DEBUG] (2) in tokio::spawn
    // 04:30:00 [DEBUG] (3) on_thread_start()

    tokio::spawn(future::lazy(|_| debug!("in tokio::spawn()")));
    debug!("in rt.block_on()");
    let r0 = future::ready("Hello from rt.block_on()").await;
    debug!("{}", r0);
    let r1 = returns_impl_future_i32().await;
    debug!("returns_impl_future_i32() -> {}", r1);
    let r2 = returns_dyn_future_i32().await;
    debug!("returns_dyn_future_i32() -> {}", r2);
    let r3 = returns_future_result().await;
    debug!("returns_future_result() -> {}", r3.unwrap());
    let r4 = returns_future_result_dyn_error().await;
    debug!("returns_future_result_dyn_error() -> {}", r4.unwrap());
    let r5 = returns_delayed_future().await;
    debug!("returns_delayed_future() -> {}", r5);
    let r6 = wait_a_sec(future::ready(42)).await;
    debug!("wait_a_sec(future::ready(42)) -> {}", r6);
    returns_future_chain().await;
}
