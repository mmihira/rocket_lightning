'use strict';
const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const UglifyJsPlugin = require('uglifyjs-webpack-plugin');
const BundleAnalyzerPlugin = require('webpack-bundle-analyzer').BundleAnalyzerPlugin;
const config = require('./config/config.js').getProperties();

module.exports = {
    entry: [
        '@babel/polyfill',
        path.join(__dirname, 'src/index.js')
    ],
    // Where you want the output to go
    mode: 'production',
    output: {
        path: path.join(__dirname, '/dist/'),
        filename: '[name]-[hash].min.js',
        publicPath: ''
    },
    plugins: [
        new webpack.LoaderOptionsPlugin({
          options: {
          eslint: {
            configFile: '.eslintrc',
            failOnWarning: false,
            failOnError: false
            }
          }
        }),
        // handles creating an index.html file and injecting assets. necessary because assets
        // change name because the hash part changes. We want hash name changes to bust cache
        // on client browsers.
        new HtmlWebpackPlugin({
            template: 'src/index.tpl.html',
            inject: 'body',
            filename: 'index.html'
        }),
        // extracts the css from the js files and puts them on a separate .css file. this is for
        // performance and is used in prod environments. Styles load faster on their own .css
        // file as they dont have to wait for the JS to load.
        //
       new MiniCssExtractPlugin({
          filename: "[name]-[hash].min.css",
          chunkFilename: "[id].css",
          allChunks: true
        }),
        new UglifyJsPlugin({
              uglifyOptions:{
                output: {
                  comments: false, // remove comments
                },
                compress: {
                  unused: true,
                  dead_code: true, // big one--strip code that will never execute
                  warnings: false, // good for prod apps so users can't peek behind curtain
                  drop_debugger: true,
                  conditionals: true,
                  evaluate: true,
                  drop_console: true, // strips console statements
                  sequences: true,
                  booleans: true,
                }
              },
            }),
        // plugin for passing in data to the js, like what NODE_ENV we are in.
        new webpack.DefinePlugin({
            'process.env.NODE_ENV': JSON.stringify('production')
        }),
        new webpack.DefinePlugin({
          RC_SERVER_URL: config.rs_server_url
        }),
        new webpack.DefinePlugin({
          RC_WS_URL: JSON.stringify(config.rc_ws_url)
        })
    ],
    node: {
          fs: 'empty',
          net: 'empty',
          tls: 'empty',
          dns: 'empty'
      },
    module: {
      rules: [
          {
              test: /\.js$/,
              exclude: /node_modules/,
              enforce: 'pre',
              loader: 'eslint-loader'
          },
          {
              test: /\.js?$/,
              exclude: /node_modules/,
              loader: 'babel-loader'
          },
          {
              test: /\.json?$/,
              loader: 'json-loader'
          },
          {
              test: /\.css/,
              use: [
                {
                  loader: MiniCssExtractPlugin.loader,
                  options: {
                    publicPath: '../'
                  }
                },
                "css-loader"
              ]
          },
          {
              test: /\.scss$/,
              loader: 'style-loader!css-loader?modules&localIdentName=[name]---[local]---[hash:base64:5]!sass-loader'
          },
          {   test: /\.woff(2)?(\?[a-z0-9#=&.]+)?$/,
              loader: 'url-loader?limit=10000&mimetype=application/font-woff-loader'
          },
          {   test: /\.(gif|ttf|eot|svg|png)(\?[a-z0-9#=&.]+)?$/,
              use: [
                    'file-loader',
                    {
                        loader: 'image-webpack-loader',
                        options: {
                            mozjpeg: {
                                progressive: false,
                                quality: 10
                            },
                            // optipng.enabled: false will disable optipng
                            optipng: {
                                enabled: false,
                            },
                            pngquant: {
                                quality: '65-90',
                                speed: 4
                            },
                            gifsicle: {
                                interlaced: false,
                            },
                            // the webp option will enable WEBP
                            webp: {
                                quality: 75
                            }
                        }
                    }
                ]
          }
      ]
    }
};
