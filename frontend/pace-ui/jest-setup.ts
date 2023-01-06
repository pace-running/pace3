import '@testing-library/jest-dom';
import {configure} from '@testing-library/react'

configure({
  getElementError: (message: string | null, container) => {
    const error = new Error(message??undefined);
    error.name = 'TestingLibraryElementError';
    error.stack = undefined;
    return error;
  },
});