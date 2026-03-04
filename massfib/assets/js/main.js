
async function kick_worker(dioxus) {
    if (dioxus == null) { return; }
    while (true) {
        let arg = await dioxus.recv();
        if (arg[0] == 9) {
            break;
        }
        //
        let worker = new Worker('./assets/worker.js');
        worker.onmessage = (evt) => {
            console.log("Data from worker received: ", evt.data);
            let r = evt.data;
            dioxus.send([r[0], r[1], r[2], r[3]]);
            worker.terminate();
        };
        worker.postMessage([arg[0], arg[1], arg[2]]);
    }
    dioxus.send("finished");
};
kick_worker(null);
