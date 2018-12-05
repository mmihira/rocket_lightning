import act from 'actions';
import { take, put, call } from 'redux-saga/effects';
import { eventChannel } from 'redux-saga';
import createSocket from 'lib/websocket';

function createSocketChannel (socket) {
  return eventChannel(
    emit => {
      const msgHandler = event => emit(event.data);
      socket.addEventListener('message', msgHandler);
      return () => socket.close();
    }
  );
}

const watchSocket = function* watchSocket () {
  yield put(act.constructingSocketRequest());
  const {socket, err} = yield call(createSocket);

  if (err) {
    console.error('Socket Connection', err);
    yield put(act.constructingSocketFail());
  } else {
    yield put(act.constructingSocketSuccess());

    const socketChannel = yield call(createSocketChannel, socket);
    while (true) {
      const payload = yield take(socketChannel);
      yield put(act.newSocketData(payload));
    }
  }
};

export {
  watchSocket
};
