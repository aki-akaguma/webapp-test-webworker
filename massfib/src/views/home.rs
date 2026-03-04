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
    let c = use_signal(|| 7);
    let mut add_r = use_signal(|| 0);
    let mut fibo_r = use_signal(|| 0);

    rsx! {
        document::Script { src: MAIN_JS }
        div { "Hello world" }
        div {
            button {
                onclick: move |_evt| async move {
                    //
                    let js = r#"{ await kick_worker(dioxus); }"#;
                    let mut eval = document::eval(js);
                    dioxus_logger::tracing::debug!("PASS 001");
                    let arg = [*a.read(), *b.read(), *c.read()];
                    dioxus_logger::tracing::debug!("send: {arg:?}");
                    eval.send(arg).unwrap();
                    dioxus_logger::tracing::debug!("PASS 002");
                    let result = eval.recv::<Vec<i32>>().await.unwrap();
                    add_r.set(result[0]);
                    fibo_r.set(result[1]);
                    dioxus_logger::tracing::debug!("PASS 003");
                },
                "KICK WORKER"
            }
        }
        br{}
        div {
            "{a} + {b} => {add_r}"
        }
        br{}
        div {
            "fibonacci {c} => {fibo_r}"
        }
    }
}
