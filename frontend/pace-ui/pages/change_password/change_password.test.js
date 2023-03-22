import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
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
  describe('form validation', () => {
    test('save button is enabled when all fields are filled', async () => {
      render(<ChangePassword />);
      await waitFor(() => {
        expect(screen.getByRole('button', { name: 'Passwort speichern' })).toBeDisabled();
      });
      await userEvent.type(screen.getByLabelText('Altes Passwort'), 'oldpassword');
      await userEvent.type(screen.getByLabelText('Neues Passwort'), 'newpassword');
      await userEvent.type(screen.getByLabelText('Neues Passwort wiederholen'), 'newpassword');
      await waitFor(() => {
        expect(screen.getByRole('button', { name: 'Passwort speichern' })).not.toBeDisabled();
      });
    });
    test('save button is disabled when new password do not match', async () => {
      render(<ChangePassword />);
      await waitFor(() => {
        expect(screen.getByRole('button', { name: 'Passwort speichern' })).toBeDisabled();
      });
      await userEvent.type(screen.getByLabelText('Altes Passwort'), 'oldpassword');
      await userEvent.type(screen.getByLabelText('Neues Passwort'), 'newpassword');
      await userEvent.type(screen.getByLabelText('Neues Passwort wiederholen'), 'newpassworddoesnotmatch');
      await waitFor(() => {
        expect(screen.getByRole('button', { name: 'Passwort speichern' })).toBeDisabled();
      });
      expect(screen.getByText('Passwörter stimmen nicht überein')).toBeInTheDocument();
    });
    test('save button is disabled when new password matches the old one', async () => {
      render(<ChangePassword />);
      await waitFor(() => {
        expect(screen.getByRole('button', { name: 'Passwort speichern' })).toBeDisabled();
      });
      await userEvent.type(screen.getByLabelText('Altes Passwort'), 'oldpassword');
      await userEvent.type(screen.getByLabelText('Neues Passwort'), 'oldpassword');
      await userEvent.type(screen.getByLabelText('Neues Passwort wiederholen'), 'oldpassword');
      await waitFor(() => {
        expect(screen.getByRole('button', { name: 'Passwort speichern' })).toBeDisabled();
      });
      expect(screen.getByText('Passwort darf nicht identisch mit dem alten Passwort sein')).toBeInTheDocument();
    });
  });
});
