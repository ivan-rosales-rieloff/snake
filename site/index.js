import("./node_modules/dcalc/dcalc.js").then((js) => {
    window.add = add;
    window.greet = greet;
});