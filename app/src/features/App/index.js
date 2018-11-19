import React from 'react';
import {connect} from 'react-redux';
import PropTypes from 'prop-types';
import act from './../../actions/index.js';
import './styles/app.css';
import TopBar from '../TopBar';

class App extends React.Component {
  constructor (props) {
    super(props);
    props.setAppDimensions(window.innerWidth, window.innerHeight);
  }

  componentDidMount () {
  }

  componentWillReceiveProps (nextProps) {
    nextProps;
  }

  render () {
    return(
      <div>
        <TopBar/>
      </div>
    );
  }
}

const mapDispatchToProps = dispatch => {
  return {
    setAppDimensions: (width, height) => dispatch(act.setAppDimensions(width, height)),
  };
};

const mapStateToProps = state => {
  state;
  return {};
};

App.propTypes = {
  setAppDimensions: PropTypes.func
};

export default connect(mapStateToProps, mapDispatchToProps)(App);
