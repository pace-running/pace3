import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { act } from 'react-dom/test-utils';
import Login from './login';
import Admin from '.';
import Finance from './finance';
import Edit from './edit';
import * as axios from 'axios';
import router from 'next/router';


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

const runner_list = [];
jest.mock('../../apis/api', () => ({
  change_payment_status: jest.fn(),
  fetchFilteredRunners: jest.fn(() => {
    return {
      status: 200,
      data: {
        stats_number: 3,
        stats_hamburg: 2,
        stats_total_donation: 20,
        runner_list
      }
    };
  })
}));

describe('checking admin main page', () => {
  beforeEach(async () => {
    await act(async ()=>render(<Admin />));
  });

  test('static elements are rendered properly', () => {
    expect(screen.getByRole('heading', { name: 'Admin' }));
    expect(screen.getByRole('heading', { name: 'Statistiken:' }));
    expect(screen.getByRole('heading', { name: 'Registrierte Läufer:' }));
  });
  
  test('check if stats are displayed correctly',()=>{
    expect(screen.getByText('Läufer gesamt: 3'));
    expect(screen.getByText('Läufer, die Hamburg starten: 2'));
    expect(screen.getByText('Spenden gesamt: 20'));
  });

  test('button linking to finance',async ()=>{
    await userEvent.click(screen.getByRole('button',{name: 'Zahlungsinformationen hochladen'}));
    expect(router.push).toHaveBeenCalledWith('/admin/finance');
  });
});
