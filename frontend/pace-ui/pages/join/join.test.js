import { describe, expect, test } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Join from '.';
import React from 'react';

describe('testing of the registration page', () => {
  beforeEach(() => {
    render(<Join />);
  });
  const user = userEvent.setup();

  describe('basic registration form displayed', () => {
    test('loads and displays join page', () => {
      expect(screen.getByText('Lauf gegen Rechts'));
      expect(screen.getByRole('heading', { name: 'Anmeldung' })).toHaveTextContent('Anmeldung');
      expect(screen.getAllByRole('heading')[1]).toHaveTextContent('Fan T-Shirt');
    });

    test('initially text fields should be empty', () => {
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
      const emailInput = screen.getByRole('textbox', { name: 'Email' });
      const emailConfirmInput = screen.getByRole('textbox', { name: 'Email wiederholen' });

      await user.type(emailInput, 'email');
      expect(screen.getByText('E-Mail muss zulässige E-Mail-Adresse sein!'));
      await user.type(emailInput, '@example.com');
      expect(screen.queryByText('E-Mail muss zulässige E-Mail-Adresse sein!')).not.toBeInTheDocument();
      expect(screen.getByText('E-Mail Adressen müssen übereinstimmen!'));
      await user.type(emailConfirmInput, 'email@example.com');

      await waitFor(() => {
        expect(screen.queryByText('E-Mail Adressen müssen übereinstimmen!')).not.toBeInTheDocument();
      });
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

      expect(donationInput).toHaveValue(10);

      donationInput.value = '';
      await expect(screen.findByText('Bitte geben Sie einen Spendenbetrag an!'));

      await user.type(donationInput, '4');
      expect(donationInput).toHaveValue(4);
      expect(screen.findByText('Die Spende muss mindestens 5€ betragen!'));
      await user.type(donationInput, '0');
      await waitFor(() => {
        expect(screen.queryByText('Die Spende muss mindestens 5€ betragen!')).not.toBeInTheDocument();
      });

      donationInput.value = '';
      await user.type(donationInput, '6,5');
      await waitFor(() => {
        expect(screen.findByText('Bitte geben Sie einen ganzzahligen Betrag an!'));
      });
    });
  });

  describe('Tshirt form displayed', () => {
    test('should display preview modal window after clicking corresponding button', async () => {
      await user.click(screen.getByRole('button', { name: 'Vorschau' }));
      expect(screen.getByText('T-Shirt Vorschau')).toBeInTheDocument();
      expect(screen.getByRole('img', { name: 'T-shirt Preview' })).toBeInTheDocument();
      await user.click(screen.getByRole('button', { name: 'Close' }));
      expect(screen.queryByText('T-Shirt Vorschau')).not.toBeInTheDocument();
      expect(screen.queryByRole('img', { name: 'T-shirt Preview' })).not.toBeInTheDocument();
    });

    test('should display modal window with size tables', async () => {
      // Can't really test the carousel behavior because jest sees all carousel pages all the time
      await user.click(screen.getByRole('button', { name: 'Größentabelle' }));
      expect(screen.getByText('T-Shirt Größentabelle')).toBeInTheDocument();
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
      expect(screen.queryByText('Modell')).not.toBeInTheDocument();
      expect(screen.queryByText('Größe')).not.toBeInTheDocument();
      expect(screen.queryByText('Lieferanschrift')).not.toBeInTheDocument();

      await user.click(screen.getByRole('switch', { name: 'Ich möchte ein T-Shirt (Kosten: 15€)' }));

      expect(screen.queryByText('Modell')).toBeInTheDocument();
      expect(screen.queryByText('Größe')).toBeInTheDocument();
      expect(screen.queryByText('Lieferanschrift')).toBeInTheDocument();

      await waitFor(() => {
        expect(screen.getAllByText('Bitte geben Sie die notwendigen Lieferinformationen an!'));
      });
    });

    test('entering shipping information hides error message', async () => {
      await user.click(screen.getByRole('switch', { name: 'Ich möchte ein T-Shirt (Kosten: 15€)' }));
      await waitFor(() => {
        expect(screen.getAllByText('Bitte geben Sie die notwendigen Lieferinformationen an!'));
      });
      await userEvent.selectOptions(screen.getByRole('combobox', { name: 'Modell' }), ['Unisex']);
      await userEvent.selectOptions(screen.getByRole('combobox', { name: 'Größe' }), ['M']);
      await userEvent.selectOptions(screen.getByRole('combobox', { name: 'Region *' }), [
        'EU-Ausland (Versandkosten: 2€)'
      ]);
      await waitFor(() => {
        userEvent.selectOptions(screen.getByRole('combobox', { name: 'Land *' }), ['Estland']);
      });
      await user.type(screen.getByRole('textbox', { name: 'Vorname *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'Nachname *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'Straße *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'Hausnummer *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'PLZ *' }), 'Niklas');
      await user.type(screen.getByRole('textbox', { name: 'Stadt *' }), 'Niklas');

      await waitFor(() => {
        expect(screen.queryByText('Bitte geben Sie die notwendigen Lieferinformationen an!')).not.toBeInTheDocument();
      });
    });

    test('t-shirt sizes dropdown should have correct options depending on the model', async () => {
      await user.click(screen.getByRole('switch', { name: 'Ich möchte ein T-Shirt (Kosten: 15€)' }));

      const modelDropdown = screen.getByRole('combobox', { name: 'Modell' });
      const sizeDropdown = screen.getByRole('combobox', { name: 'Größe' });

      await userEvent.selectOptions(modelDropdown, ['Unisex']);
      expect(sizeDropdown.children[1]).toHaveTextContent('S');
      expect(sizeDropdown.children[2]).toHaveTextContent('M');
      expect(sizeDropdown.children[3]).toHaveTextContent('L');
      expect(sizeDropdown.children[4]).toHaveTextContent('XL');
      expect(sizeDropdown.children[5]).toHaveTextContent('XXL');
      await userEvent.selectOptions(sizeDropdown, ['XXL']);
      await userEvent.selectOptions(modelDropdown, ['Tailliert']);
      expect(screen.queryByText('XXL')).not.toBeInTheDocument();

      expect(sizeDropdown.children[1]).toHaveTextContent('S');
      expect(sizeDropdown.children[2]).toHaveTextContent('M');
      expect(sizeDropdown.children[3]).toHaveTextContent('L');
      expect(sizeDropdown.children[4]).toHaveTextContent('XL');

      expect(sizeDropdown.children).toHaveLength(5);
    });
  });

  describe('submit button', () => {
    test('submit button is initially disabled', () => {
      expect(screen.getByRole('button', { name: 'Weiter' })).toBeDisabled();
    });

    test('accepting terms and conditions enables submit button', async () => {
      await user.click(screen.getByText('Mir ist bewusst,', { exact: false }));
      expect(screen.getByRole('button', { name: 'Weiter' })).toBeEnabled();
    });

    test('link to privacy notice', () => {
      expect(screen.getByRole('link', { name: 'Datenschutzbestimmungen' })).toHaveAttribute('href', '/privacy_notice');
    });
  });
});