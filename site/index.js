import("./node_modules/dcalc/dcalc.js").then((js) => {
    window.add = js.add;
    window.greet = js.greet;
});