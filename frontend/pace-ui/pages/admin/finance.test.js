import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Finance from './finance';
import router from 'next/router';
import { uploadPaymentCSV, getAllRejectedTransactions } from '../../apis/api';

jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));

jest.mock('../../apis/api', () => ({
  uploadPaymentCSV: jest.fn(),
  getAllRejectedTransactions: jest.fn()
}));

describe('test the finance page', () => {
  afterEach(() => {
    jest.clearAllMocks();
  });

  afterAll(() => {
    jest.restoreAllMocks();
  });

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

    expect(uploadPaymentCSV).toHaveBeenCalledWith(file);
  });

  test('number of accepted and rejected transactions from last upload is displayed in the table', async () => {
    const response = {
      status: 200,
      data: [27,43]
    };

    uploadPaymentCSV.mockResolvedValue(response);

    render(<Finance />);
    const str = JSON.stringify('test');
    const blob = new Blob([str]);
    const file = new File([blob], 'values.csv', {
      type: 'application/CSV'
    });

    await userEvent.upload(screen.getByLabelText('Hier .csv-Datei einfügen:'), file);
    await userEvent.click(screen.getByRole('button', { name: 'Einlesen' }));

    expect(screen.getByText('Upload erfolgreich, 27 Transaktionen bestätigt und 43 abgelehnt!'));
  });

  test('if upload of csv goes wrong, error message is displayed',async ()=>{
    const response = {
      status: 401,
      data: [27,43]
    };

    uploadPaymentCSV.mockResolvedValue(response);

    render(<Finance />);
    const str = JSON.stringify('test');
    const blob = new Blob([str]);
    const file = new File([blob], 'values.csv', {
      type: 'application/CSV'
    });

    await userEvent.upload(screen.getByLabelText('Hier .csv-Datei einfügen:'), file);
    await userEvent.click(screen.getByRole('button', { name: 'Einlesen' }));

    screen.getByText('Beim Upload ist etwas schiefgelaufen!');
  });

});
