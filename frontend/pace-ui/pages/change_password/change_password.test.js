import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { act } from 'react-dom/test-utils';
import ChangePassword from '.';
import router from 'next/router';

jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));
describe('change password page', () => {
  test('old password field is present', async () => {
    render(<ChangePassword />);
    await waitFor(() => {
      expect(screen.getByLabelText('Altes Passwort')).toBeInTheDocument();
    });
  });
  test('new password field is present', async () => {
    render(<ChangePassword />);
    await waitFor(() => {
      expect(screen.getByLabelText('Neues Passwort')).toBeInTheDocument();
    });
  });
  test('new password repeat field is present', async () => {
    render(<ChangePassword />);
    await waitFor(() => {
      expect(screen.getByLabelText('Neues Passwort wiederholen')).toBeInTheDocument();
    });
  });
  test('set password', async () => {
    render(<ChangePassword />);
    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Passwort speichern' })).toBeDisabled();
    });
  });
  test('button back to admin page works', async () => {
    render(<ChangePassword />);
    await userEvent.click(screen.getByRole('button', { name: 'Zurück zum Adminbereich' }));
    expect(router.push).toHaveBeenCalledWith('/admin');
  });
});
