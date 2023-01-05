import '@testing-library/jest-dom';
import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Join from '.';
import React from 'react';


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
      await new Promise((r) => setTimeout(r, 0));
      await expect(screen.queryByText('E-Mail Adressen müssen übereinstimmen!')).not.toBeInTheDocument();
    });

    test('dropdown menu should display obligatory options', () => {
      const startingPointDropdown = screen.getByRole('combobox', { name: 'Von wo wirst du laufen? *' });
      const runningLevelDropdown = screen.getByRole('combobox', { name: 'Wie schätzt du dein Laufniveau ein? *' });

      expect(startingPointDropdown).toHaveTextContent('Bitte auswählen');
      expect(runningLevelDropdown).toHaveTextContent('Bitte auswählen');

      expect(startingPointDropdown.children[1]).toHaveTextContent('in Hamburg bei der Alster vor Ort');
      expect(startingPointDropdown.children[2]).toHaveTextContent('woanders');

      expect(runningLevelDropdown.children[1]).toHaveTextContent('Ich laufe selten');
      expect(runningLevelDropdown.children[2]).toHaveTextContent('Ich laufe gelegentlich bis regelmäßig');
      expect(runningLevelDropdown.children[3]).toHaveTextContent('Ich laufe häufig und ambitioniert');


    });
    test('should check edge cases for donation field', async () => {
      const donationInput = screen.getByRole('spinbutton', { name: 'Ich möchte spenden (mindestens 5€)' });
      const user = userEvent.setup();

      expect(donationInput).toHaveValue(10);

      (donationInput as HTMLInputElement).value = '';
      await expect(screen.findByText('Bitte geben Sie einen Spendenbetrag an!'));

      await user.type(donationInput, '4');
      expect(donationInput).toHaveValue(4);
      await expect(screen.findByText('Die Spende muss mindestens 5€ betragen!'));

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




