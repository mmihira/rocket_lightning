import {createStore, applyMiddleware} from 'redux';
import rootReducer from 'reducers.js';
import createSagaMiddleware from 'redux-saga';
import root from 'sagas';
import createSagaMonitor from 'lib/sagaMonitor/index.js';

const monitor = createSagaMonitor();
const sagaMiddleware = createSagaMiddleware({sagaMonitor: monitor});
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
