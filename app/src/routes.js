import React from 'react';
import { Route, Switch } from 'react-router-dom';
import About from './features/About';

export default (
  function s() {
    return (
      <Switch>
        <Route path="/about" component={About} />
      </Switch>
    );
  }()
);
