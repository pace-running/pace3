import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import ChangeTheme from '../../pages/admin/changeTheme';
import { getThemeVar } from '../../utility/theme';

jest.mock('../../utility/theme', () => ({
  getThemeVar: jest.fn()
}));

describe('change theme page', () => {
  test('initially, input fields display current values', async () => {
    getThemeVar.mockImplementation(key => {
      if (key === 'event_name') return 'Test Title';
      if (key === 'event_description') return 'Test Description';
      if (key === 'is_registration_open') return 'true';
      if (key === 'enable_tshirts') return 'false';
      if (key === 'closed_registration_message') return 'test message';
    });
    render(<ChangeTheme />);
    screen.debug();
    expect(screen.queryByLabelText('Titel des Events:')).toHaveValue('Test Title');
    expect(screen.getByLabelText('Beschreibung des Events:')).toHaveValue('Test Description');
    expect(screen.getByLabelText('Nachricht, falls Registrierung geschlossen ist:')).toHaveValue('test message');
    expect(screen.getByLabelText('Ist die Registrierung geöffnet?')).toBeChecked();
    expect(screen.getByLabelText('Werden T-Shirts angeboten?')).not.toBeChecked();
  });
});
