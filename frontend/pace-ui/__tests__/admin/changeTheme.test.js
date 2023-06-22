import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import ChangeTheme from '../../pages/admin/changeTheme';
import { getThemeVar } from '../../utility/theme';
import { updateTheme } from '../../apis/api';

jest.mock('../../utility/theme', () => ({
  getThemeVar: jest.fn()
}));

jest.mock('../../apis/api', () => ({
  updateTheme: jest.fn()
}));

describe('change theme page', () => {
  test('initially, input fields display current values', async () => {
    getThemeVar.mockImplementation(key => {
      if (key === 'event_name') return 'Test Title';
      if (key === 'event_description') return 'Test Description';
      if (key === 'is_registration_open') return 'true';
      if (key === 'enable_tshirts') return 'false';
      if (key === 'closed_registration_message') return 'test message';
      if (key === 'decentral_signup') return 'true';
    });
    render(<ChangeTheme />);
    expect(screen.getByLabelText('Titel des Events:')).toHaveValue('Test Title');
    expect(screen.getByLabelText('Beschreibung des Events:')).toHaveValue('Test Description');
    expect(screen.getByLabelText('Nachricht, falls Registrierung geschlossen ist:')).toHaveValue('test message');
    expect(screen.getByLabelText('Ist die Registrierung geöffnet?')).toBeChecked();
    expect(screen.getByLabelText('Werden T-Shirts angeboten?')).not.toBeChecked();
    expect(screen.getByLabelText('Kann auch dezentral gelaufen werden?')).toBeChecked();
  });

  test('submitting form sends correct api request', async () => {
    getThemeVar.mockImplementation(key => {
      if (key === 'event_name') return 'Test Title';
      if (key === 'event_description') return 'Test Description';
      if (key === 'is_registration_open') return 'true';
      if (key === 'enable_tshirts') return 'false';
      if (key === 'closed_registration_message') return 'test message';
      if (key === 'decentral_signup') return 'true';
    });
    render(<ChangeTheme />);

    const titleInput = screen.getByLabelText('Titel des Events:');
    const descriptionInput = screen.getByLabelText('Beschreibung des Events:');
    const messageInput = screen.getByLabelText('Nachricht, falls Registrierung geschlossen ist:');

    await userEvent.clear(titleInput);
    await userEvent.type(titleInput, 'title1');

    await userEvent.clear(descriptionInput);
    await userEvent.type(descriptionInput, 'description2');

    await userEvent.clear(messageInput);
    await userEvent.type(messageInput, 'message3');

    await userEvent.click(screen.getByLabelText('Werden T-Shirts angeboten?'));
    await userEvent.click(screen.getByRole('button', { name: 'Änderungen speichern!' }));

    expect(updateTheme).toHaveBeenCalledWith({
      eventTitle: 'title1',
      eventDescription: 'description2',
      closedRegistrationMessage: 'message3',
      isRegistrationOpen: true,
      tshirtsEnabled: true,
      decentralSignupEnabled: true,
    });
  });
});
