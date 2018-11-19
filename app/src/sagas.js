import { all } from 'redux-saga/effects';
import * as sagaFns from './sagas/index.js';
import * as _ from 'lodash';

export default function* rootSaga () {
  yield all(_.values(sagaFns).map(fn => fn()));
}
