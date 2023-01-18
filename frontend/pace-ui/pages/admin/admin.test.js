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

let renderedPage;

describe('Log in page', () => {
  beforeEach(() => {
    renderedPage=render(<Login />);
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

describe('admin main page', () => {
  beforeEach(async () => {
    await act(async ()=>render(<Admin />));
  });

  test('check if stats are displayed correctly',()=>{
    expect(screen.getByRole('total-runners').textContent).toBe('3')
    expect(screen.getByText('Läufer, die Hamburg starten: 2'));
    expect(screen.getByText('Spenden gesamt: 20'));
  });

  test('should check the button linking to the finance page',async ()=>{
    await userEvent.click(screen.getByRole('button',{name: 'Zahlungsinformationen hochladen'}));
    expect(router.push).toHaveBeenCalledWith('/admin/finance');
  });
});
