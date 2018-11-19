import * as NameSpaces from '../../../lib/api/nameSpaces.js';
import { call, put, take } from 'redux-saga/effects';
import { expectSaga } from 'redux-saga-test-plan';
import { parameterPanelInit } from '../../../sagas/index.js';
import sinon from 'sinon';
import * as types from '../../../features/ParametersPanel/actions/types.js';

// Keep for future reference
//
// test('parameterPanelInit', () => {
//   sinon.stub(NameSpaces, 'getNameSpaces').callsFake(() => {
//     return Promise.resolve(['space1', 'space2']);
//   });

//   sinon.stub(NameSpaces, 'getNameSpace').callsFake((name) => {
//     return {
//       nameSpace: name,
//       functions: [{name: 'f1'}, {name: 'f2'}]
//     };
//   });

//   return expectSaga(parameterPanelInit)
//     .dispatch({
//       type: types.PARAMETER_PANEL_INIT,
//     })
//     .put({
//       type: types.SAGA_SET_NS_CHILDREN,
//       data: [
//         {
//           name: 'space1',
//           type: types.NODE_TYPES.NAMESPACE,
//           path: 'children[0]',
//           children: [
//             {name: 'f1', type: types.NODE_TYPES.FUNCTION, nSpaceParent: 'space1', path: 'children[0].children[0]'},
//             {name: 'f2', type: types.NODE_TYPES.FUNCTION, nSpaceParent: 'space1', path: 'children[0].children[1]'}
//           ]
//         },
//         {
//           name: 'space2',
//           type: types.NODE_TYPES.NAMESPACE,
//           path: 'children[1]',
//           children: [
//             {name: 'f1', type: types.NODE_TYPES.FUNCTION, nSpaceParent: 'space2', path: 'children[1].children[0]'},
//             {name: 'f2', type: types.NODE_TYPES.FUNCTION,nSpaceParent: 'space2', path: 'children[1].children[1]'}
//           ]
//         }
//       ]
//     })
//     .run();
// });
