import { routerReducer as routing } from 'react-router-redux';
import { combineReducers } from 'redux';
import * as reducers from './reducers/index.js';

const rootReducer = combineReducers(
  Object.assign(
    { routing },
    reducers
  )
);

export default rootReducer;
