/**
 * DEV only saga action monitor
 */
const ENABLE_ACTION_DISPATCH_LOG = true;
const ENABLE_EFFECT_RESOLVED_LOG = false;
const ENABLE_EFFECT_TRIGGERED_LOG = false;
const ENABLE_EFFECT_REJECTED_LOG = false;
const ENABLE_EFFECT_CANCELED_LOG = false;

export default function createSagaMonitor () {
  function effectTriggered (effect) {
    if (ENABLE_EFFECT_TRIGGERED_LOG) {
      console.warn(effect);
    }
  }

  function effectResolved (effectId, result) {
    if (ENABLE_EFFECT_RESOLVED_LOG) {
      console.warn(effectId, result);
    }
  }

  function effectRejected (effectId, error) {
    if (ENABLE_EFFECT_REJECTED_LOG) {
      console.warn(effectId, error);
    }
  }

  function effectCancelled (effectId) {
    if (ENABLE_EFFECT_CANCELED_LOG) {
      console.warn(effectId);
    }
  }


  function actionDispatched (action) {
    if (ENABLE_ACTION_DISPATCH_LOG) {
      console.warn('ACTION DISPATCH', action);
    }
  }

  return {
    effectTriggered, effectResolved, effectRejected, effectCancelled, actionDispatched
  };
}
