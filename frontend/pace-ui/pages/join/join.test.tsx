import '@testing-library/jest-dom';
import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Join from '.';
import React from 'react';
import { wait } from '@testing-library/user-event/dist/types/utils';

describe('testing of the registration page', () => {
  beforeEach(() => {
    render(<Join />);
  });

  describe('basic information displayed', () => {
    test('loads and displays join page', () => {
      expect(screen.getByText('Lauf gegen Rechts'));
      expect(screen.getByRole('heading', { name: 'Anmeldung' })).toHaveTextContent('Anmeldung');
      expect(screen.getAllByRole('heading')[1]).toHaveTextContent('Fan T-Shirt');
    });

    test('initially text fields should be empty', () => {
      expect(screen.getByRole('textbox', { name: 'Vorname (erscheint auf der Startnummer)' })).toHaveTextContent('');
    });

    test('email input field should display correct error messages', async () => {
      const emailInput = screen.getByRole('textbox', { name: 'Email' });
      const emailConfirmInput = screen.getByRole('textbox', { name: 'Email wiederholen' });
      const user = userEvent.setup();

      await user.type(emailInput, 'email');
      expect(screen.getByText('E-Mail muss zulässige E-Mail-Adresse sein!'));
      await user.type(emailInput, '@example.com');
      expect(screen.queryByText('E-Mail muss zulässige E-Mail-Adresse sein!')).not.toBeInTheDocument();
      expect(screen.getByText('E-Mail Adressen müssen übereinstimmen!'));
      await user.type(emailConfirmInput, 'email@example.com');
      // empty timeout needed, otherwise test does not wait for error message to disappear
      await new Promise((r) => setTimeout(r,0));
      await expect(screen.queryByText('E-Mail Adressen müssen übereinstimmen!')).not.toBeInTheDocument();
    });

  });

  describe('submit button', () => {
    test('submit button is initially disabled', () => {
      expect(screen.getByRole('button', { name: 'Weiter' })).toBeDisabled();
    });

    test('accepting terms and conditions enables submit button', async () => {
      const user = userEvent.setup();
      await user.click(screen.getByText('Mir ist bewusst,', { exact: false }));
      expect(screen.getByRole('button', { name: 'Weiter' })).toBeEnabled();
    });
  });
});
