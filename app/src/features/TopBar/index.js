import React from 'react';
import { Pane, Heading} from 'evergreen-ui';
import '@fortawesome/fontawesome-free/css/all.css';
import elIcon from '../../assets/elephant.svg';

class TopBar extends React.Component {
  constructor (props) {
    super(props);
  }

  render () {
    return(
      <Pane
        margin=""
        top="0px"
        width="100%"
        height="55px"
        display="flex"
        alignItems="center"
        justifyContent="flex-start"
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
          border="none"
        >
          <img
            src={elIcon}
            style={{color: 'white', paddingLeft: '5px', width: '40px'}}
            title="Icon by Freepik from www.flaticon.com">
          </img>
          <img
            src={elIcon}
            style={{color: 'white', paddingLeft: '5px', width: '40px'}}
            title="Icon by Freepik from www.flaticon.com">
          </img>
          <img
            src={elIcon}
            style={{color: 'white', paddingLeft: '5px', width: '40px'}}
            title="Icon by Freepik from www.flaticon.com">
          </img>
        </Pane>
        <Heading
          size={600}
          paddingLeft="40px"
          color="white">
          rc-ui
        </Heading>
      </Pane>
    );
  }
}

export default TopBar;
