import {createStore, applyMiddleware} from 'redux';
import rootReducer from '../reducers.js';
import createSagaMiddleware from 'redux-saga';
import root from '../sagas';

const sagaMiddleware = createSagaMiddleware();
export function configureStore (initialState) {
  const store = createStore(
    rootReducer,
    initialState,
    applyMiddleware(sagaMiddleware)
  );

  sagaMiddleware
    .run(root);

  return store;
}
