
async function kick_worker(dioxus) {
    if (dioxus == null) { return; }
    let arg = await dioxus.recv();
    //
    let worker = new Worker('./assets/worker.js');
    worker.onmessage = (evt) => {
        console.log("Data from worker received: ", evt.data);
        dioxus.send([evt.data[0], evt.data[1]]);
    };
    worker.postMessage([arg[0], arg[1], arg[2]]);
};
kick_worker(null);
