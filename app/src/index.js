import React from 'react';
import { render } from 'react-dom';
import Root from './features/Root/Root.js';
import {store} from  './store.js';

render(
  <Root store={store} history={history} />,
  document.getElementById('root')
);
