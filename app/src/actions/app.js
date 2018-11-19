import * as types from './types';

const setAppDimensions = (innerWidth, innerHeight) => ({
  type: types.SET_APP_DIMENSIONS,
  innerWidth,
  innerHeight
});

export {
  setAppDimensions
};
