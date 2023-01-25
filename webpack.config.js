//Webpack config file for the project
const path = require('path');
const HTMLWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    //for the entry we specify the main js file, which isn't the rust app. We keep them separated and thus, the html and js files are in the public folder
    entry: './public/main.js',
    //where we're going to store the bundled output
    output: {
        //we use the node path function to get the current dirname, and we output to the dist folder
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js'
    },
    plugins: [
        //we import this plugin to give webpack html support and make it serve our simple HTML page
        new HTMLWebpackPlugin({
            template: './public/index.html'
        })
    ]
}