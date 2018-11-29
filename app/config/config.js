/**
 * API configuration
 */

const convict = require('convict');

const config = convict({
  env: {
    doc: 'The application environment.',
    format: ['production', 'development', 'sandbox'],
    default: 'development',
    env: 'NODE_ENV'
  },
  express_port: {
    doc: 'The port the UI listens to.',
    format: 'port',
    default: 8071,
    env: 'EXPRESS_PORT',
    arg: 'express_port'
  },
  bind_address: {
    doc: 'The address the UI listens to.',
    format: '*',
    default: '0.0.0.0',
    env: 'BIND_ADDRESS',
    arg: 'bind_address'
  },
  rc_server_url: {
    doc: 'The pgFaas API URL.',
    format: 'url',
    default: 'localhost:8000',
    env: 'RC_SERVER_URL',
    arg: 'rc_server_url'
  },
  rc_ws_url: {
    doc: 'The host:port where the websocket server is located',
    format: 'url',
    default: 'localhost:3012',
    env: 'RC_WS_URL',
    arg: 'rc_ws_url'
  }
});

config.loadFile('./config/' + config.get('env') + '.json');
config.validate({allowed: 'strict'});

module.exports = config;
