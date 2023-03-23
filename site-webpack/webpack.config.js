const path = require('path');

module.exports = {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js",
        clean: {
            keep: "index.html",
        },
    },
    mode: "development",
    experiments: {
        asyncWebAssembly: true,
    },
    devServer: {
        static: './dist',
    },
};
