import React from 'react';
import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import Login from './login';
import Admin from '.';
import Finance from './finance';
import Edit from './edit';
import * as axios from 'axios';
import userEvent from '@testing-library/user-event';

jest.mock('axios');
jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));

describe('checking log in page', () => {
  beforeEach(() => {
    render(<Login />);
  });

  test('checking if username and password fields are visible', () => {
    expect(screen.getByRole('textbox', { name: 'Username' }));
    expect(screen.getByLabelText('Passwort'));
  });

  test('looking for Login button and checking if it submits correctly', async () => {
    axios.post.mockResolvedValue(null);
    process.env.NEXT_PUBLIC_API_URL = 'mockURL';
    await userEvent.type(screen.getByLabelText('Username'), 'User1');
    await userEvent.type(screen.getByLabelText('Passwort'), 'User25');
    await userEvent.click(screen.getByRole('button', { name: 'Login' }));
    expect(axios.post).toHaveBeenCalledWith('mockURL/api/admin/login', { password: 'User25', username: 'User1' });
  });
});
