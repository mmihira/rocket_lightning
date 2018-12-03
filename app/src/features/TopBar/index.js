import React from 'react';
import * as types from 'actions/types';
import PropTypes from 'prop-types';
import {connect} from 'react-redux';
import { Pane, Heading, Icon} from 'evergreen-ui';
import '@fortawesome/fontawesome-free/css/all.css';

class TopBar extends React.Component {
  constructor (props) {
    super(props);
  }

  render () {
    const { socketConstructionState } = this.props.appInit;

    const sockConDepProps = (function sockConDepProps () {
      switch (socketConstructionState) {
        case types.CONSTRUCTING_SOCKET_REQUEST:
          return {
            socketIconColor: 'warning',
            statusColor: 'orange',
            statusInfo: 'connecting ...'
          };
        case types.CONSTRUCTING_SOCKET_FAIL:
          return {
            socketIconColor: 'danger',
            statusColor: 'ghostwhite',
            statusInfo: 'disconnected ...'
          };
        case types.CONSTRUCTING_SOCKET_SUCCESSFULL:
          return {
            socketIconColor: 'success',
            statusColor: 'white',
            statusInfo: 'connected'
          };
        default:
          return {};
      }
    }());

    return(
      <Pane
        margin=""
        top="0px"
        width="100%"
        height="55px"
        display="flex"
        alignItems="center"
        justifyContent="space-between"
        background="#336791"
        border="none"
      >
        <Pane
          margin=""
          top="0px"
          width="10%"
          display="flex"
          alignItems="center"
          justifyContent="center"
          border="none">
          <Heading
            size={600}
            paddingLeft="40px"
            color="white">
            rc-ui
          </Heading>
        </Pane>
        <Pane
          margin=""
          color="white"
          fontSize="small"
          top="0px"
          width="15%"
          display="flex"
          alignItems="center"
          justifyContent="center"
          border="none">
          <Pane
            color={sockConDepProps.statusColor}
            marginRight="5px">
            { sockConDepProps.statusInfo }
          </Pane>
          <Icon icon="social-media" color={sockConDepProps.socketIconColor} />
        </Pane>
      </Pane>
    );
  }
}

TopBar.propTypes = {
  appInit: PropTypes.object
};

const mapStateToProps = state => {
  const { appInit } = state;
  return { appInit };
};

export default connect(mapStateToProps, null)(TopBar);
