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
  pgfaas_api_url: {
    doc: 'The pgFaas API URL.',
    format: 'url',
    default: 'http://pgfaas.aurin.org.au/api',
    env: 'PGFAAS_API_URL',
    arg: 'pgfaas_api_url'
  },
  disable_delete: {
    doc: 'Disable the delete function and delete namespace functionalities.',
    format: 'Boolean',
    default: false,
    env: 'DISABLE_DELETE',
    arg: 'disable_delete'
  },
  protected_namespaces: {
    doc: 'Prevent update of specified namespaces',
    format: 'String',
    default: "[]",
    env: 'PROTECTED_NAMESPACES',
    arg: 'protected_namespaces'
  }
});

config.loadFile('./config/' + config.get('env') + '.json');
config.validate({allowed: 'strict'});

module.exports = config;
