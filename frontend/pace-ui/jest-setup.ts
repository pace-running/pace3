import '@testing-library/jest-dom';
import { configure } from '@testing-library/react';

jest.mock('./utility/theme', () => ({
  getThemeVar: jest.fn().mockImplementation(key => {
    return `[${key}]`;
  })
}));

configure({
  getElementError: (message: string | null) => {
    const error = new Error(message ?? undefined);
    error.name = 'TestingLibraryElementError';
    error.stack = undefined;
    return error;
  }
});
