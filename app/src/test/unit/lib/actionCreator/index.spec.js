import makeActionCreator from 'lib/actionCreator';

describe('makeActionCreator', () => {
  describe('with a simple type parameter only', () => {
    test('has correct type', () => {
      const TYPE = 'TYPE1';
      const action = makeActionCreator(TYPE);
      expect(action().type).toBe(TYPE);
    });
  });

  describe('with multiple args', () => {
    const TYPE = 'TYPE1';
    const arg1 = 'value';
    const arg2 = 'value2';
    const action = makeActionCreator(TYPE, arg1, arg2);

    test('has correct type', () => {
      expect(action().type).toBe(TYPE);
    });

    test('has correct first arg', () => {
      expect(action(20, 30)[arg1]).toBe(20);
    });

    test('has correct second arg', () => {
      expect(action(20, 30)[arg2]).toBe(30);
    });
  });
});
