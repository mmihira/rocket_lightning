import * as types from 'actions/types';

const initialData = {
  vol: [1, 2, 2, 3, 4, 5, 7]
};

export default (state = initialData, action) => {
  switch (action.type) {
    case types.SOCKET_DATA:
      return {
        ...state,
        vol: JSON.parse(action.data).map(e => e.vol)
      };
    default:
      return state;
  }
};
