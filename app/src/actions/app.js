import * as types from './types';
import makeActionCreator from 'lib/actionCreator';

const exports = {
  setAppDimensions: makeActionCreator(types.SET_APP_DIMENSIONS, 'innerWidth', 'innerHeight'),
  constructingSocketRequest: makeActionCreator(types.CONSTRUCTING_SOCKET_REQUEST),
  constructingSocketSuccess: makeActionCreator(types.CONSTRUCTING_SOCKET_SUCCESSFULL),
  constructingSocketFail: makeActionCreator(types.CONSTRUCTING_SOCKET_FAIL)
};

export default exports;
