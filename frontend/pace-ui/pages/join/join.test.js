import { describe, expect, test, xtest } from '@jest/globals';
import { findByText, render, screen, waitFor, waitForElementToBeRemoved, act } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Join from '.';
import React from 'react';

jest.setTimeout(30000); // Added higher timeout so the pipeline tests do not fail because of timeouts

describe('testing of the registration page', () => {
  async function testNameFieldValidation(user, inputField, errorMessage) {
    await waitFor(() => expect(screen.queryByText(errorMessage)).not.toBeInTheDocument());

    await user.type(inputField, '123');
    expect(await screen.findByText(errorMessage)).toBeInTheDocument();

    await user.clear(inputField);
    await waitFor(() => expect(screen.queryByText(errorMessage)).not.toBeInTheDocument());
    await user.type(inputField, '!@?');

    await user.clear(inputField);
    await waitFor(() => expect(screen.queryByText(errorMessage)).not.toBeInTheDocument());
    await user.type(inputField, 'Name123');
    expect(await screen.findByText(errorMessage)).toBeInTheDocument();

    await user.clear(inputField);
    await user.type(inputField, 'Sönke-Maël');
    await waitFor(() => expect(screen.queryByText(errorMessage)).not.toBeInTheDocument());

    await user.clear(inputField);
    await user.type(inputField, 'Büşra Maria');
    await waitFor(() => expect(screen.queryByText(errorMessage)).not.toBeInTheDocument());
  }

  describe('basic registration form displayed', () => {
    test('loads and displays join page', () => {
      render(<Join />);

      expect(screen.getByText('Lauf gegen Rechts')).toBeInTheDocument();
      expect(screen.getByRole('heading', { name: 'Anmeldung' })).toHaveTextContent('Anmeldung');
      expect(screen.getAllByRole('heading')[1]).toHaveTextContent('Fan T-Shirt');
    });

    test('initially text fields should be empty', () => {
      render(<Join />);

      const names = [
        'Vorname (erscheint auf der Startnummer)',
        'Nachname',
        'Team Name (erscheint auf der Startnummer)',
        'Email',
        'Email wiederholen'
      ];
      for (const name of names) {
        expect(screen.getByRole('textbox', { name: name })).toHaveTextContent('');
      }
    });

    test('email input field should display correct error messages', async () => {
      const user = userEvent.setup();
      render(<Join />);

      const emailInput = screen.getByRole('textbox', { name: 'Email' });
      const emailConfirmInput = screen.getByRole('textbox', { name: 'Email wiederholen' });

      await user.type(emailInput, 'email');
      expect(await screen.findByText('E-Mail muss zulässige E-Mail-Adresse sein!')).toBeInTheDocument();
      await user.type(emailInput, '@example.com');
      await waitFor(() =>
        expect(screen.queryByText('E-Mail muss zulässige E-Mail-Adresse sein!')).not.toBeInTheDocument()
      );
      expect(screen.getByText('E-Mail Adressen müssen übereinstimmen!'));
      await user.type(emailConfirmInput, 'email@example.com');
      await waitFor(() => {
        expect(screen.queryByText('E-Mail Adressen müssen übereinstimmen!')).not.toBeInTheDocument();
      });
    });

    test('dropdown menu should display obligatory options', () => {
      render(<Join />);

      const startingPointDropdown = screen.getByRole('combobox', { name: 'Von wo wirst du laufen? *' });
      const runningLevelDropdown = screen.getByRole('combobox', { name: 'Wie schätzt du dein Laufniveau ein? *' });

      expect(runningLevelDropdown).toHaveTextContent('Bitte auswählen');

      expect(startingPointDropdown.children[1]).toHaveTextContent('in Hamburg bei der Alster vor Ort');
      expect(startingPointDropdown.children[2]).toHaveTextContent('woanders');

      expect(runningLevelDropdown.children[1]).toHaveTextContent('Ich laufe selten');
      expect(runningLevelDropdown.children[2]).toHaveTextContent('Ich laufe gelegentlich bis regelmäßig');
      expect(runningLevelDropdown.children[3]).toHaveTextContent('Ich laufe häufig und ambitioniert');
    });

    test('should check edge cases for donation field', async () => {
      const user = userEvent.setup();
      render(<Join />);

      const donationInput = screen.getByRole('spinbutton', { name: 'Ich möchte spenden (mindestens 5€)' });
      expect(donationInput).toHaveValue(10);

      await user.clear(donationInput);
      expect(await screen.findByText('Bitte geben Sie einen Spendenbetrag an!')).toBeInTheDocument();

      await user.type(donationInput, '4');
      expect(await screen.findByText('Die Spende muss mindestens 5€ betragen!')).toBeInTheDocument();
      await user.type(donationInput, '0');
      await waitFor(() =>
        expect(screen.queryByText('Die Spende muss mindestens 5€ betragen!')).not.toBeInTheDocument()
      );

      await user.clear(donationInput);
      await user.type(donationInput, '6.5');
      expect(donationInput).toHaveValue(6.5);
      expect(await screen.findByText('Bitte geben Sie einen ganzzahligen Betrag an!')).toBeInTheDocument();
    });

    test('should display error if first name contains numbers', async () => {
      const user = userEvent.setup();
      render(<Join />);

      const firstNameInput = screen.getByRole('textbox', { name: 'Vorname (erscheint auf der Startnummer)' });
      const errorMessage = 'Vorname darf keine Zahlen oder Sonderzeichen enthalten!';

      await testNameFieldValidation(user, firstNameInput, errorMessage);
    });

    test('should display error if last name contains numbers', async () => {
      const user = userEvent.setup();
      render(<Join />);

      const lastNameInput = screen.getByRole('textbox', { name: 'Nachname' });
      const errorMessage = 'Nachname darf keine Zahlen oder Sonderzeichen enthalten!';

      await testNameFieldValidation(user, lastNameInput, errorMessage);
    });
  });

  describe('Tshirt form displayed', () => {
    test('should display preview modal window after clicking corresponding button', async () => {
      const user = userEvent.setup();
      render(<Join />);

      await user.click(screen.getByRole('button', { name: 'Vorschau' }));
      await waitFor(() => expect(screen.getByText('T-Shirt Vorschau')).toBeInTheDocument());
      expect(screen.getByRole('img', { name: 'T-shirt Preview' })).toBeInTheDocument();
      await user.click(screen.getByRole('button', { name: 'Close' }));
      await waitFor(() => expect(screen.queryByText('T-Shirt Vorschau')).not.toBeInTheDocument());
      expect(screen.queryByRole('img', { name: 'T-shirt Preview' })).not.toBeInTheDocument();
    });

    test('should display modal window with size tables', async () => {
      const user = userEvent.setup();
      render(<Join />);

      // Can't really test the carousel behavior because jest sees all carousel pages all the time
      await user.click(screen.getByRole('button', { name: 'Größentabelle' }));
      expect(await screen.findByText('T-Shirt Größentabelle')).toBeInTheDocument();
      expect(screen.getByText('Tailliert')).toBeInTheDocument();
      expect(screen.getAllByText('XL')).toHaveLength(2);
      expect(screen.getByRole('button', { name: 'Next' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Previous' })).toBeInTheDocument();
      expect(screen.getByText('Unisex')).toBeInTheDocument();
      expect(screen.getByText('XXL')).toBeInTheDocument();

      await user.click(screen.getByRole('button', { name: 'Close' }));
      expect(screen.queryByText('T-Shirt Größentabelle')).not.toBeInTheDocument();
    });

    test('Toggling the Tshirt option shows / hides the shipping information fields', async () => {
      const user = userEvent.setup();
      render(<Join />);

      expect(screen.queryByText('Modell')).not.toBeInTheDocument();
      expect(screen.queryByText('Größe')).not.toBeInTheDocument();
      expect(screen.queryByText('Lieferanschrift')).not.toBeInTheDocument();

      await user.click(screen.getByRole('switch', { name: 'Ich möchte ein T-Shirt (Kosten: 15€)' }));
      await waitFor(() => expect(screen.queryByText('Modell')).toBeInTheDocument());
      expect(screen.queryByText('Größe')).toBeInTheDocument();
      expect(screen.queryByText('Lieferanschrift')).toBeInTheDocument();

      expect(
        (await screen.findAllByText('Bitte geben Sie die notwendigen Lieferinformationen an!')).length
      ).toBeGreaterThan(0);
    });

    test('entering shipping information hides error message', async () => {
      const user = userEvent.setup();
      render(<Join />);

      await screen.findByText('Ich möchte ein T-Shirt (Kosten: 15€)');
      await user.click(screen.getByRole('switch', { name: 'Ich möchte ein T-Shirt (Kosten: 15€)' }));
      expect(
        (await screen.findAllByText('Bitte geben Sie die notwendigen Lieferinformationen an!')).length
      ).toBeGreaterThan(0);
      await user.selectOptions(screen.getByRole('combobox', { name: 'Modell' }), ['Unisex']);
      await user.selectOptions(screen.getByRole('combobox', { name: 'Größe' }), ['M']);
      await user.selectOptions(screen.getByRole('combobox', { name: 'Region *' }), ['EU-Ausland (Versandkosten: 2€)']);
      await user.selectOptions(screen.getByRole('combobox', { name: 'Land *' }), ['Estland']);
      await user.type(screen.getByRole('textbox', { name: 'Vorname *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'Nachname *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'Straße *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'Hausnummer *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'PLZ *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'Stadt *' }), 'Niklas');
      await waitFor(() =>
        expect(screen.queryByText('Bitte geben Sie die notwendigen Lieferinformationen an!')).not.toBeInTheDocument()
      );
    });

    test('adding numbers or special characters to shipping address first name field displays error', async () => {
      const user = userEvent.setup();
      render(<Join />);

      await user.click(screen.getByRole('switch', { name: 'Ich möchte ein T-Shirt (Kosten: 15€)' }));
      expect(
        (await screen.findAllByText('Bitte geben Sie die notwendigen Lieferinformationen an!')).length
      ).toBeGreaterThan(0);

      const firstNameInput = screen.getByRole('textbox', { name: 'Vorname *' });
      const errorMessage = 'Vorname darf keine Zahlen oder Sonderzeichen enthalten!';
      await testNameFieldValidation(user, firstNameInput, errorMessage);
    });

    test('adding numbers or special characters to shipping address last name field displays error', async () => {
      const user = userEvent.setup();
      render(<Join />);

      await user.click(screen.getByRole('switch', { name: 'Ich möchte ein T-Shirt (Kosten: 15€)' }));
      expect(
        (await screen.findAllByText('Bitte geben Sie die notwendigen Lieferinformationen an!')).length
      ).toBeGreaterThan(0);

      const lastNameInput = screen.getByRole('textbox', { name: 'Nachname *' });
      const errorMessage = 'Nachname darf keine Zahlen oder Sonderzeichen enthalten!';
      await testNameFieldValidation(user, lastNameInput, errorMessage);
    });

    test('t-shirt sizes dropdown should have correct options depending on the model', async () => {
      const user = userEvent.setup();
      render(<Join />);

      await user.click(screen.getByRole('switch', { name: 'Ich möchte ein T-Shirt (Kosten: 15€)' }));
      expect(
        (await screen.findAllByText('Bitte geben Sie die notwendigen Lieferinformationen an!')).length
      ).toBeGreaterThan(0);

      const modelDropdown = screen.getByRole('combobox', { name: 'Modell' });
      const sizeDropdown = screen.getByRole('combobox', { name: 'Größe' });

      await user.selectOptions(modelDropdown, ['Unisex']);
      expect(sizeDropdown.children[1]).toHaveTextContent('S');
      expect(sizeDropdown.children[2]).toHaveTextContent('M');
      expect(sizeDropdown.children[3]).toHaveTextContent('L');
      expect(sizeDropdown.children[4]).toHaveTextContent('XL');
      expect(sizeDropdown.children[5]).toHaveTextContent('XXL');
      await user.selectOptions(sizeDropdown, ['XXL']);
      await user.selectOptions(modelDropdown, ['Tailliert']);
      await waitFor(() => expect(screen.queryByText('XXL')).not.toBeInTheDocument());

      expect(sizeDropdown.children[1]).toHaveTextContent('S');
      expect(sizeDropdown.children[2]).toHaveTextContent('M');
      expect(sizeDropdown.children[3]).toHaveTextContent('L');
      expect(sizeDropdown.children[4]).toHaveTextContent('XL');

      expect(sizeDropdown.children).toHaveLength(5);
    });
  });

  describe('submit button', () => {
    test('submit button is initially disabled', () => {
      render(<Join />);
      expect(screen.getByRole('button', { name: 'Weiter' })).toBeDisabled();
    });

    test('accepting terms and conditions enables submit button', async () => {
      const user = userEvent.setup();
      render(<Join />);

      await user.click(screen.getByText('Mir ist bewusst,', { exact: false }));
      await user.selectOptions(screen.getByRole('combobox', { name: 'Von wo wirst du laufen? *' }), 'hamburg');
      await user.selectOptions(
        screen.getByRole('combobox', { name: 'Wie schätzt du dein Laufniveau ein? *' }),
        'often'
      );
      expect(await screen.findByRole('button', { name: 'Weiter' })).toBeEnabled();
    });

    test('link to privacy notice', () => {
      render(<Join />);
      expect(screen.getByRole('link', { name: 'Datenschutzbestimmungen' })).toHaveAttribute('href', '/privacy_notice');
    });
  });
});
