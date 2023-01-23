import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { act } from 'react-dom/test-utils';

import Finance from './finance';
import router from 'next/router';
import { upload_payment_csv } from '../../apis/api';

jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));

jest.mock('../../apis/api', () => ({
  upload_payment_csv: jest.fn()
}));

describe('test the finance page', () => {
  test('button back to admin page works', async () => {
    render(<Finance />);
    await userEvent.click(screen.getByRole('button', { name: 'Zurück zum Adminbereich' }));
    expect(router.push).toHaveBeenCalledWith('/admin');
  });

  test('clicking the button without uploading a file results in error message', async () => {
    render(<Finance />);
    expect(screen.queryByText('Bitte wähle zunächst eine Datei aus!')).not.toBeInTheDocument();
    await userEvent.click(screen.getByRole('button', { name: 'Einlesen' }));
    expect(screen.queryByText('Bitte wähle zunächst eine Datei aus!')).toBeInTheDocument();
  });

  test('uploading wrong file type results in error message', async () => {
    render(<Finance />);
    const str = JSON.stringify('test');
    const blob = new Blob([str]);
    const file = new File([blob], 'values.json', {
      type: 'application/JSON'
    });

    await userEvent.upload(screen.getByLabelText('Hier .csv-Datei einfügen:'), file);
    expect(screen.getByText('Die Datei muss im .csv-Format sein!')).toBeInTheDocument();
  });

  test('uploading a file works', async () => {
    render(<Finance />);
    const str = JSON.stringify('test');
    const blob = new Blob([str]);
    const file = new File([blob], 'values.csv', {
      type: 'application/CSV'
    });

    await userEvent.upload(screen.getByLabelText('Hier .csv-Datei einfügen:'), file);
    await userEvent.click(screen.getByRole('button', { name: 'Einlesen' }));

    expect(upload_payment_csv).toHaveBeenCalledWith(file);
  });
});
