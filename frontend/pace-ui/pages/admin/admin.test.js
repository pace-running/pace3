import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { act } from 'react-dom/test-utils';

import router from 'next/router';
import { change_payment_status, fetchFilteredRunners } from '../../apis/api';
import Admin from '.';

jest.mock('axios');
jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));

jest.mock('../../apis/api', () => ({
  change_payment_status: jest.fn(),
  fetchFilteredRunners: jest.fn()
}));

const runner_list = [];

describe('admin main page', () => {
  const apiResponse = {
    status: 200,
    data: {
      stats_number: 3,
      stats_hamburg: 2,
      stats_total_donation: 20,
      runner_list
    }
  };

  afterEach(() => {
    jest.clearAllMocks();
  });

  afterAll(() => {
    jest.restoreAllMocks();
  });

  test('check if stats are displayed correctly', async () => {
    fetchFilteredRunners.mockResolvedValueOnce(apiResponse);
    await act(async () => render(<Admin />));
    expect(screen.getByTestId('total-runners-p').textContent).toBe('Läufer gesamt: 3');
    expect(screen.getByText('Läufer, die Hamburg starten: 2'));
    expect(screen.getByText('Spenden gesamt: 20'));
  });

  test('should check the button linking to the finance page', async () => {
    fetchFilteredRunners.mockResolvedValueOnce(apiResponse);
    await act(async () => render(<Admin />));
    await userEvent.click(screen.getByRole('button', { name: 'Zahlungsinformationen hochladen' }));
    expect(router.push).toHaveBeenCalledWith('/admin/finance');
  });

  test('unauthenticated users will be redirected to login', async () => {
    console.log(fetchFilteredRunners);
    fetchFilteredRunners.mockResolvedValueOnce({
      status: 401,
      data: {}
    });
    await act(async () => render(<Admin />));
    expect(router.push).toHaveBeenCalledWith('/admin/login');
  });
});
