module.exports = {
  verbose: true,
  testRegex: '(/app/test/(unit)/.*|(\\.|/)(test|spec))\\.jsx?$',
  transform: {
    "^.+\\.jsx?$": "babel-jest"
  }
};
