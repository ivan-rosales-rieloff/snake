const path = require("path");
module.exports = {
  entry: "./site/index.js",
  output: {
    path: path.resolve(__dirname + 'site', "dist"),
    filename: "index.js",
  },
  mode: "development",
};