import * as types from 'actions/types';
import * as _ from 'lodash';

const initialData = {
  clientInnerWidth: 0,
  clientInnerHeight: 0
};

export default (state = initialData, action) => {
  switch (action.type) {
    case types.SET_APP_DIMENSIONS:
      return _.merge(
        {},
        {
          clientInnerWidth: action.innerWidth,
          clientInnerHeight: action.innerHeight
        }
      );
    default:
      return state;
  }
};
