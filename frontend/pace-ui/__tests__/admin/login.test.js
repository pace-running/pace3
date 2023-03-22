import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Login from '../../pages/admin/login';

import * as axios from 'axios';

jest.mock('axios');
jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));

describe('Log in page', () => {
  beforeEach(() => {
    render(<Login />);
  });

  test('should submit username and password to the public api login url when clicking login button', async () => {
    axios.post.mockResolvedValue(null);
    process.env.NEXT_PUBLIC_API_URL = 'mockURL';
    await userEvent.type(screen.getByLabelText('Username'), 'User1');
    await userEvent.type(screen.getByLabelText('Passwort'), 'User25');
    await userEvent.click(screen.getByRole('button', { name: 'Login' }));
    expect(axios.post).toHaveBeenCalledWith('mockURL/api/admin/login', { password: 'User25', username: 'User1' });
  });
});
