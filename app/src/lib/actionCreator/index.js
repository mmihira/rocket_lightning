/**
 * Create a simple action creator
 * @param type {String} A type from lib/actions/types.js
 * @param argName {Array} arguments as as array of strings
 */
export default (type, ...argNames) => {
  return function (...args) {
    const action = { type };
    argNames.forEach((arg, index) => {
      action[argNames[index]] = args[index];
    });
  };
};
