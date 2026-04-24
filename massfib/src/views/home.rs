use dioxus::prelude::*;

#[cfg(not(feature = "web"))]
use std::time::Instant;
#[cfg(feature = "web")]
use web_time::Instant;

const MAIN_JS: Asset = asset!("/assets/js/main.js", AssetOptions::js().with_minify(false));

// This is necessary. Without it, `dx build --web --release` will not copy `worker.jp`.
const _WORKER_JS: Asset = asset!(
    "/assets/js/worker.js",
    AssetOptions::js()
        .with_minify(false)
        .with_hash_suffix(false)
);

#[component]
pub fn Home() -> Element {
    let mut single_elapsed = use_signal(|| 0.0);
    let mut multi_elapsed = use_signal(|| 0.0);
    let add_r = use_signal(|| 0);
    let fibo37_r = use_signal(|| 0);
    let fibo38_r = use_signal(|| 0);
    let fibo39_r = use_signal(|| 0);
    let fibo40_r = use_signal(|| 0);
    let fibo41_r = use_signal(|| 0);

    rsx! {
        document::Script { src: MAIN_JS }
        div { " " }
        div {
            button {
                onclick: move |_evt| async move {
                    single_elapsed.set(0.0);
                    let start = Instant::now();
                    single_proc(add_r, fibo37_r, fibo38_r, fibo39_r, fibo40_r, fibo41_r).await;
                    let elapsed = (start.elapsed().as_millis() as f32) / 1000.0;
                    single_elapsed.set(elapsed);
                },
                "Single Worker"
            }
            span {
                " {single_elapsed} sec"
            }
        }
        br{}
        div {
            button {
                onclick: move |_evt| async move {
                    multi_elapsed.set(0.0);
                    let start = Instant::now();
                    multi_proc(add_r, fibo37_r, fibo38_r, fibo39_r, fibo40_r, fibo41_r).await;
                    let elapsed = (start.elapsed().as_millis() as f32) / 1000.0;
                    multi_elapsed.set(elapsed);
                },
                "Multi WORKER"
            }
            span {
                " {multi_elapsed} sec"
            }
        }
        br{}
        div {
            "2 + 3 => {add_r}"
        }
        br{}
        div { "fibonacci 37 => {fibo37_r}" }
        div { "fibonacci 38 => {fibo38_r}" }
        div { "fibonacci 39 => {fibo39_r}" }
        div { "fibonacci 40 => {fibo40_r}" }
        div { "fibonacci 41 => {fibo41_r}" }
    }
}

async fn single_proc(
    mut add_r: Signal<i32>,
    mut fibo37_r: Signal<i32>,
    mut fibo38_r: Signal<i32>,
    mut fibo39_r: Signal<i32>,
    mut fibo40_r: Signal<i32>,
    mut fibo41_r: Signal<i32>,
) {
    add_r.set(0);
    fibo37_r.set(0);
    fibo38_r.set(0);
    fibo39_r.set(0);
    fibo40_r.set(0);
    fibo41_r.set(0);
    //
    let js = r#"{ await kick_worker(dioxus); }"#;
    let mut eval = document::eval(js);
    dioxus::logger::tracing::debug!("PASS 001");
    //
    let arg = [1, 2, 3];
    dioxus::logger::tracing::debug!("send 1: {arg:?}");
    eval.send(arg).unwrap();
    let result = eval.recv::<Vec<i32>>().await.unwrap();
    add_r.set(result[3]);
    //
    let arg = [2, 37, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    let result = eval.recv::<Vec<i32>>().await.unwrap();
    fibo37_r.set(result[3]);
    //
    let arg = [2, 38, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    let result = eval.recv::<Vec<i32>>().await.unwrap();
    fibo38_r.set(result[3]);
    //
    let arg = [2, 39, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    let result = eval.recv::<Vec<i32>>().await.unwrap();
    fibo39_r.set(result[3]);
    //
    let arg = [2, 40, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    let result = eval.recv::<Vec<i32>>().await.unwrap();
    fibo40_r.set(result[3]);
    //
    let arg = [2, 41, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    let result = eval.recv::<Vec<i32>>().await.unwrap();
    fibo41_r.set(result[3]);
    //
    dioxus::logger::tracing::debug!("PASS 002");
    //
    eval.send([9]).unwrap();
    dioxus::logger::tracing::debug!("PASS 004");
    let _ = eval.recv::<String>().await.unwrap();
    dioxus::logger::tracing::debug!("PASS 005");
}

async fn multi_proc(
    mut add_r: Signal<i32>,
    mut fibo37_r: Signal<i32>,
    mut fibo38_r: Signal<i32>,
    mut fibo39_r: Signal<i32>,
    mut fibo40_r: Signal<i32>,
    mut fibo41_r: Signal<i32>,
) {
    add_r.set(0);
    fibo37_r.set(0);
    fibo38_r.set(0);
    fibo39_r.set(0);
    fibo40_r.set(0);
    fibo41_r.set(0);
    //
    let js = r#"{ await kick_worker(dioxus); }"#;
    let mut eval = document::eval(js);
    dioxus::logger::tracing::debug!("PASS 001");
    //
    let arg = [1, 2, 3];
    dioxus::logger::tracing::debug!("send 1: {arg:?}");
    eval.send(arg).unwrap();
    //
    let arg = [2, 41, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    let arg = [2, 40, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    let arg = [2, 39, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    let arg = [2, 38, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    let arg = [2, 37, 0];
    dioxus::logger::tracing::debug!("send 2: {arg:?}");
    eval.send(arg).unwrap();
    //
    dioxus::logger::tracing::debug!("PASS 002");
    //
    for i in 0..6 {
        let result = eval.recv::<Vec<i32>>().await.unwrap();
        let sw = result[0];
        if sw == 1 {
            add_r.set(result[3]);
        } else if sw == 2 {
            let a = result[1];
            if a == 37 {
                fibo37_r.set(result[3]);
            } else if a == 38 {
                fibo38_r.set(result[3]);
            } else if a == 39 {
                fibo39_r.set(result[3]);
            } else if a == 40 {
                fibo40_r.set(result[3]);
            } else if a == 41 {
                fibo41_r.set(result[3]);
            }
        }
        dioxus::logger::tracing::debug!("PASS 003: {i}");
    }
    eval.send([9]).unwrap();
    dioxus::logger::tracing::debug!("PASS 004");
    let _ = eval.recv::<String>().await.unwrap();
    dioxus::logger::tracing::debug!("PASS 005");
}
