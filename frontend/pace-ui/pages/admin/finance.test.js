import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Finance from './finance';
import router from 'next/router';
import { uploadPaymentCSV } from '../../apis/api';

jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));

jest.mock('../../apis/api', () => ({
  uploadPaymentCSV: jest.fn()
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

  test('faulty transactions are displayed in the table', async () => {
    const response = {
      status: 200,
      data: [
        {
          runner_ids: ['21', '42'],
          reason_for_payment: 'LGR-DFSKF, LGR-OBZSA',
          amount: '25',
          expected_amount: '37'
        }
      ]
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

    const table = document.getElementById('runnersTable');

    const headers = within(table).getAllByRole('columnheader');
    const firstRowCells = within(table).getAllByRole('row')[1].children;

    expect(headers[0]).toHaveTextContent('Teilnehmenden ID');
    expect(firstRowCells[0]).toHaveTextContent('21, 42');

    expect(headers[1]).toHaveTextContent('Verwendungszweck');
    expect(firstRowCells[1]).toHaveTextContent('LGR-DFSKF, LGR-OBZSA');

    expect(headers[2]).toHaveTextContent('erhaltener Betrag');
    expect(firstRowCells[2]).toHaveTextContent('25');

    expect(headers[3]).toHaveTextContent('erwarteter Betrag');
    expect(firstRowCells[3]).toHaveTextContent('37');
  });
});
