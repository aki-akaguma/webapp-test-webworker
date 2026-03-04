importScripts("pkg/wasm-worker.js")
const { greet, add, fibonacci } = wasm_bindgen;

onmessage = (e) => {
    console.log("Worker: Message recived");
    let sw = e.data[0];
    let a = e.data[1];
    let b = e.data[2];

    (async () => {
        await wasm_bindgen("pkg/wasm-worker_bg.wasm")

        if (sw == 1) {
            const result_add = add(a, b);
            if (!isNaN(result_add)) {
                postMessage([1, a, b, result_add]);
                return;
            }
        } else if (sw == 2) {
            const result_fibo = fibonacci(a);
            if (!isNaN(result_fibo)) {
                postMessage([2, a, b, result_fibo]);
                return;
            }
        }
        //console.log("Worker: Posting message back to main script");
        postMessage("Please write three numbers: [sw, a, b] sw: 1 or 2");
    })();
};
