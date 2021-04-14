const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = {
	entry: './src/bootstrap.js',
	output: {
		filename: 'main.js',
		path: path.resolve(__dirname, 'dist'),
	},
	mode: "development",
	experiments: {
		syncWebAssembly: true,
	},
	plugins: [new HtmlWebpackPlugin()],
};
