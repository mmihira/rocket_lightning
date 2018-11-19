import * as types from './../actions/types.js';
import { take } from 'redux-saga/effects';

const initApp = function* _initApp () {
  yield take(types.SET_APP_DIMENSIONS);
};

export {
  initApp
};

