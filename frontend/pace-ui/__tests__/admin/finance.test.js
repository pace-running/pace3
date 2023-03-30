import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import Finance from '../../pages/admin/finance';
import router from 'next/router';
import { uploadPaymentCSV, getAllRejectedTransactions, logOutUser } from '../../apis/api';
import { act } from 'react-dom/test-utils';

jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));

jest.mock('../../apis/api', () => ({
  uploadPaymentCSV: jest.fn(),
  getAllRejectedTransactions: jest.fn(),
  logOutUser: jest.fn()
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

  test('logout button logs out the user and re-routes to the login page', async () => {
    render(<Finance />);
    await userEvent.click(screen.getByRole('button', { name: 'Ausloggen' }));
    expect(logOutUser).toHaveBeenCalled();
    expect(router.push).toHaveBeenCalledWith('/admin/login');
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

  test('number of accepted and rejected transactions from last upload is displayed', async () => {
    const response = {
      status: 200,
      data: [27, 43]
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

  test('if upload of csv goes wrong, error message is displayed', async () => {
    const response = {
      status: 401,
      data: [27, 43]
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

  test('rejected transactions are displayed in the table correctly', async () => {
    const apiResponse = {
      status: 200,
      data: [
        {
          id: 55,
          runner_ids: '105',
          reasons_for_payment: 'LGR-YPKDM, LGR-YPKPP',
          payment_amount: '25',
          expected_amount: '25, 25',
          currency: 'EUR',
          date_of_payment: '26.01.2023',
          payer_name: 'Test McTesty',
          iban: 'DE57500105175574174785'
        }
      ]
    };
    getAllRejectedTransactions.mockResolvedValue(apiResponse);
    await act(async () => render(<Finance />));

    const table = screen.getByRole('table');
    const headers = within(table).getAllByRole('columnheader');
    const firstRow = within(table).getAllByRole('row')[1];
    const firstRowCells = firstRow.children;
    expect(headers[0]).toHaveTextContent('Datum');
    expect(firstRowCells[0]).toHaveTextContent('26.01.2023');
    expect(headers[1]).toHaveTextContent('Teilnehmenden IDs');
    expect(firstRowCells[1]).toHaveTextContent('105');

    expect(headers[2]).toHaveTextContent('Verwendungszweck');
    expect(firstRowCells[2]).toHaveTextContent('LGR-YPKDM, LGR-YPKPP');

    expect(headers[3]).toHaveTextContent('Betrag');
    expect(firstRowCells[3]).toHaveTextContent('25');
    expect(headers[4]).toHaveTextContent('Erwarteter Betrag');
    expect(firstRowCells[4]).toHaveTextContent('25, 25');
    expect(headers[5]).toHaveTextContent('Währung');
    expect(firstRowCells[5]).toHaveTextContent('EUR');
    expect(headers[6]).toHaveTextContent('Name');
    expect(firstRowCells[6]).toHaveTextContent('Test McTesty');

    expect(headers[7]).toHaveTextContent('IBAN');
    expect(firstRowCells[7]).toHaveTextContent('DE57500105175574174785');
  });
});
