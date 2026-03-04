use dioxus::prelude::*;

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
    let a = use_signal(|| 2);
    let b = use_signal(|| 3);
    let mut add_r = use_signal(|| 0);
    let mut fibo37_r = use_signal(|| 0);
    let mut fibo38_r = use_signal(|| 0);
    let mut fibo39_r = use_signal(|| 0);
    let mut fibo40_r = use_signal(|| 0);
    let mut fibo41_r = use_signal(|| 0);

    rsx! {
        document::Script { src: MAIN_JS }
        div { "Hello world" }
        div {
            button {
                onclick: move |_evt| async move {
                    add_r.set(0);
                    fibo37_r.set(0);
                    fibo38_r.set(0);
                    fibo39_r.set(0);
                    fibo40_r.set(0);
                    fibo41_r.set(0);
                    //
                    let js = r#"{ await kick_worker(dioxus); }"#;
                    let mut eval = document::eval(js);
                    dioxus_logger::tracing::debug!("PASS 001");
                    //
                    let arg = [1, *a.read(), *b.read()];
                    dioxus_logger::tracing::debug!("send 1: {arg:?}");
                    eval.send(arg).unwrap();
                    //
                    let arg = [2, 41, 0];
                    dioxus_logger::tracing::debug!("send 2: {arg:?}");
                    eval.send(arg).unwrap();
                    let arg = [2, 40, 0];
                    dioxus_logger::tracing::debug!("send 2: {arg:?}");
                    eval.send(arg).unwrap();
                    let arg = [2, 39, 0];
                    dioxus_logger::tracing::debug!("send 2: {arg:?}");
                    eval.send(arg).unwrap();
                    let arg = [2, 38, 0];
                    dioxus_logger::tracing::debug!("send 2: {arg:?}");
                    eval.send(arg).unwrap();
                    let arg = [2, 37, 0];
                    dioxus_logger::tracing::debug!("send 2: {arg:?}");
                    eval.send(arg).unwrap();
                    //
                    dioxus_logger::tracing::debug!("PASS 002");
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
                        dioxus_logger::tracing::debug!("PASS 003: {i}");
                    }
                    eval.send([9]).unwrap();
                    dioxus_logger::tracing::debug!("PASS 004");
                    let _ = eval.recv::<String>().await.unwrap();
                    dioxus_logger::tracing::debug!("PASS 005");
                },
                "KICK WORKER"
            }
        }
        br{}
        div {
            "{a} + {b} => {add_r}"
        }
        br{}
        div { "fibonacci 37 => {fibo37_r}" }
        div { "fibonacci 38 => {fibo38_r}" }
        div { "fibonacci 39 => {fibo39_r}" }
        div { "fibonacci 40 => {fibo40_r}" }
        div { "fibonacci 41 => {fibo41_r}" }
    }
}
