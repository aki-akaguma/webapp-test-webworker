importScripts("pkg/wasm-worker.js")
const { greet, add, fibonacci } = wasm_bindgen;

onmessage = (e) => {
    console.log("Worker: Message recived");
    let a = e.data[0];
    let b = e.data[1];
    let c = e.data[2];

    (async () => {
        await wasm_bindgen("pkg/wasm-worker_bg.wasm")

        const result_add = add(a, b);
        if (isNaN(result_add)) {
            postMessage("Please write two numbers");
            return;
        }
        const result_fibo = fibonacci(c);
        if (isNaN(result_fibo)) {
            postMessage("Please write three numbers");
            return;
        }
        //console.log("Worker: Posting message back to main script");
        postMessage([result_add, result_fibo]);
    })();
};
