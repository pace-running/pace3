import '@testing-library/jest-dom';
import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Join from '.';

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
