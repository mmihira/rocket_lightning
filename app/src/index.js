import React from 'react';
import { render } from 'react-dom';
import Root from './root/Root';
import {store} from  './store.js';

render(
  <Root store={store} history={history} />,
  document.getElementById('root')
);
