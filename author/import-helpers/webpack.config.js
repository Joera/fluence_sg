const path = require("path");
const webpack = require("webpack");
const isProduction = process.env.NODE_ENV == "production";

const config = (env) =>  {

  return {
    entry: "./src/index.ts",
    output: {
      path: path.resolve(__dirname, "build/"),
      chunkFilename: 'obsidian_import_helpers.js',
      filename: 'obsidian_import_helpers.js',
    },
    plugins: [],
    module: {
      rules: [
        {
          test: /\.(ts|tsx)$/i,
          loader: "ts-loader",
          exclude: ["/node_modules/"],
        }
      ],
    },
    resolve: {
      extensions: [".ts",".js"]
    }
  }
};

module.exports = (env) => {

  let c = config(env);

  if (isProduction) {
    c.mode = "production";
  } else {
    c.mode = "development";
  }
  return c;
};
