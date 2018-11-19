let PROTECTED_NAMESPACES_CONFIG = [];
try {
  PROTECTED_NAMESPACES_CONFIG = JSON.parse(PROTECTED_NAMESPACES);
} catch (e) {
  console.warn('Error setting protected namespaces', e);
}

module.exports = {
  DISABLE_DELETE: DISABLE_DELETE,
  NODE_ENV: process.env.NODE_ENV,
  PROTECTED_NAMESPACES: PROTECTED_NAMESPACES_CONFIG
};
