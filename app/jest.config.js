const path = require('path');

module.exports = {
  verbose: true,
  testRegex: '(/app/test/(unit)/.*|(\\.|/)(test|spec))\\.jsx?$',
  modulePaths: [
    path.resolve('./src'),
  ],
  transform: {
    "^.+\\.jsx?$": "babel-jest"
  }
};
