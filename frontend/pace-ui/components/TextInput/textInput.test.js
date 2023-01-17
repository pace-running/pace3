import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';

import TextInput from '.';

describe('test text input field', () => {
  test('email text input has type "email"', () => {
    render(<TextInput type='email' name='email-input' label='email-label' />);
    const inputField = screen.getByRole('textbox');
    expect(inputField).toHaveProperty('type', 'email');
    expect(screen.getByLabelText('email-label')).toEqual(inputField);
  });

  test('text input has type "text"', () => {
    render(<TextInput type='text' name='text-input' label='text-label' />);
    const inputField = screen.getByRole('textbox');
    expect(inputField).toHaveProperty('type', 'text');
    expect(screen.getByLabelText('text-label')).toEqual(inputField);
  });

  test('number input has type "number"', () => {
    render(<TextInput type='number' name='number-input' label='number-label' />);
    const inputField = screen.getByRole('spinbutton');
    expect(inputField).toHaveProperty('type', 'number');
    expect(screen.getByLabelText('number-label')).toEqual(inputField);
  });

  test('password text input has type "password"', () => {
    render(<TextInput type='password' name='password-input' label='password-label' />);
    const inputField = screen.getByLabelText('password-label');
    expect(inputField).toHaveProperty('type', 'password');
  });

  test('input field has correct interaction when typing', async () => {
    const handleChange = jest.fn();
    render(<TextInput type='text' name='text-input' label='input-label' onChange={handleChange} />);
    const input = screen.getByRole('textbox');
    await userEvent.type(input, 'hello');
    expect(handleChange).toHaveBeenCalledTimes(5);
    expect(input.value).toBe('hello');
  });
});
