const isDevelopment = process.env.NODE_ENV === 'development'

const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
    entry: "./src/index.tsx",
    target: "web",
    mode: isDevelopment ? "development" : "production",
    output: {
        path: path.resolve(__dirname, "build"),
        filename: "bundle.js",
    },
    resolve: {
        extensions: [".js", ".jsx", ".json", ".ts", ".tsx", ".scss"],
        alias: {
            "components": path.resolve(__dirname, 'src/components'),
            "routes": path.resolve(__dirname, 'src/routes'),
            "assets": path.resolve(__dirname, 'src/assets')
        }
    },
    module: {
        rules: [
            {
                test: /\.scss$/,
                use: [
                    {
                        loader: "style-loader"
                    },
                    {
                        loader: "css-loader",
                        options: {
                            modules: true
                        }
                    },
                    {
                        loader: "sass-loader"
                    }
                ]
            },
            {
                test: /\.(ts|tsx)$/,
                loader: "ts-loader",
                exclude: /node_modules/,
            }
        ],
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.resolve(__dirname, "src", "index.html"),
        }),
        // new MiniCssExtractPlugin({
        //     filename: isDevelopment ? '[name].css' : '[name].[hash].css',
        //     chunkFilename: isDevelopment ? '[id].css' : '[id].[hash].css'
        // })
    ],
};