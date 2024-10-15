export default {
    entry: './dist/tucan-injector.js',
    experiments: {
        asyncWebAssembly: true
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: "asset/inline",
            },
        ],
    },
};