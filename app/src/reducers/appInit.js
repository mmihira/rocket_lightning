import * as types from 'actions/types';

const initialData = {
  clientInnerWidth: 0,
  clientInnerHeight: 0,
  socketConstructionState: types.CONSTRUCTING_SOCKET_REQUEST
};

export default (state = initialData, action) => {
  switch (action.type) {
    case types.SET_APP_DIMENSIONS:
      return {
        ...state,
        clientInnerWidth: action.innerWidth,
        clientInnerHeight: action.innerHeight
      };
    case types.CONSTRUCTING_SOCKET_REQUEST:
      return {
        ...state,
        socketConstructionState: action.type
      };
    case types.CONSTRUCTING_SOCKET_SUCCESSFULL:
      return {
        ...state,
        socketConstructionState: action.type
      };
    case types.CONSTRUCTING_SOCKET_FAIL:
      return {
        ...state,
        socketConstructionState: action.type
      };
    default:
      return state;
  }
};
