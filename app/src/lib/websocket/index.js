import * as CONFIG from '../../config';

function createSocket () {
  try {
    const socket = new WebSocket(`wrs://${CONFIG.RC_WS_URL}`);
    return {socket};
  } catch (err) {
    return {err};
  }
}

export default createSocket;
