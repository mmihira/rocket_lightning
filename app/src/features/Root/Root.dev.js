import PropTypes from 'prop-types';
import React from 'react';
import {Provider} from 'react-redux';
import App from 'features/App';

export default function Root ({store}) {
  return (
    <Provider store={store}>
      <App />
    </Provider>
  );
}

Root.propTypes = {
  store: PropTypes.object.isRequired
};
