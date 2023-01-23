import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { act } from 'react-dom/test-utils';

import router from 'next/router';
import { change_payment_status, fetchFilteredRunners } from '../../apis/api';
import Admin from '.';

jest.mock('axios');
jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));

jest.mock('../../apis/api', () => ({
  change_payment_status: jest.fn(),
  fetchFilteredRunners: jest.fn()
}));

describe('admin main page', () => {
  const apiResponse = {
    status: 200,
    data: {
      stats_number: 3,
      stats_hamburg: 2,
      stats_total_donation: 20,
      runner_list: []
    }
  };

  afterEach(() => {
    jest.clearAllMocks();
  });

  afterAll(() => {
    jest.restoreAllMocks();
  });

  test('check if stats are displayed correctly', async () => {
    fetchFilteredRunners.mockResolvedValueOnce(apiResponse);
    await act(async () => render(<Admin />));
    expect(screen.getByTestId('total-runners-p').textContent).toBe('Läufer gesamt: 3');
    expect(screen.getByText('Läufer, die Hamburg starten: 2'));
    expect(screen.getByText('Spenden gesamt: 20'));
  });

  test('should check the button linking to the finance page', async () => {
    fetchFilteredRunners.mockResolvedValueOnce(apiResponse);
    await act(async () => render(<Admin />));
    await userEvent.click(screen.getByRole('button', { name: 'Zahlungsinformationen hochladen' }));
    expect(router.push).toHaveBeenCalledWith('/admin/finance');
  });

  test('unauthenticated users will be redirected to login', async () => {
    fetchFilteredRunners.mockRejectedValueOnce();
    await act(async () => render(<Admin />));
    expect(router.push).toHaveBeenCalledWith('/admin/login');
    expect(screen.getByText('Seite lädt...'));
  });

  test('runners are displayed in table', async () => {
    apiResponse.data.runner_list = [
      {
        donation: '5',
        email: 'test@example.com',
        firstname: 'Hans',
        id: 1,
        lastname: 'Meyer',
        payment_confirmation_mail_sent: true,
        payment_status: true,
        reason_for_payment: 'LGR-YPKDM',
        running_level: 'sometimes',
        start_number: 6,
        starting_point: 'other',
        team: 'FC St. Pauli',
        tshirt_cost: '15',
        verification_code: 'ogVXRyN8GpMSXUNV3VSx1ZBoUYwK95Sa8u'
      },
      {
        donation: '33',
        email: 'test5@example.com',
        firstname: 'Testy',
        id: 5,
        lastname: 'McTest',
        payment_confirmation_mail_sent: false,
        payment_status: false,
        reason_for_payment: 'LGR-YPKDX',
        running_level: 'sometimes',
        start_number: 66,
        starting_point: 'other',
        team: 'FC St. Pauli II',
        tshirt_cost: '33',
        verification_code: 'ogVXRyN8GpMSXUNV3VSx1ZBoUYwK95Sa8x'
      }
    ];
    fetchFilteredRunners.mockResolvedValueOnce(apiResponse);
    await act(async () => render(<Admin />));

    const table = screen.getByRole('table');
    const headers = within(table).getAllByRole('columnheader');
    const firstRow = within(table).getAllByRole('row')[1];
    const firstRowCells = firstRow.children;
    const secondRowCells = within(table).getAllByRole('row')[2].children;

    // relies on getAllByRole to return elements in order of appearance
    expect(headers[0]).toHaveTextContent('ID');
    expect(firstRowCells[0]).toHaveTextContent('1');
    expect(secondRowCells[0]).toHaveTextContent('5');

    expect(headers[1]).toHaveTextContent('Startnummer');
    expect(firstRowCells[1]).toHaveTextContent('6');
    expect(secondRowCells[1]).toHaveTextContent('66');

    expect(headers[2]).toHaveTextContent('Name');
    expect(firstRowCells[2]).toHaveTextContent('Hans Meyer');
    expect(secondRowCells[2]).toHaveTextContent('Testy McTest');

    expect(headers[3]).toHaveTextContent('Team');
    expect(firstRowCells[3]).toHaveTextContent('FC St. Pauli');
    expect(secondRowCells[3]).toHaveTextContent('FC St. Pauli II');

    expect(headers[4]).toHaveTextContent('E-mail');
    expect(firstRowCells[4]).toHaveTextContent('test@example.com');
    expect(secondRowCells[4]).toHaveTextContent('test5@example.com');

    expect(headers[5]).toHaveTextContent('Spende');
    expect(firstRowCells[5]).toHaveTextContent('5');
    expect(secondRowCells[5]).toHaveTextContent('33');

    expect(headers[6]).toHaveTextContent('Verwendungszweck');
    expect(firstRowCells[6]).toHaveTextContent('LGR-YPKDM');
    expect(secondRowCells[6]).toHaveTextContent('LGR-YPKDX');
  });

  test('table contains correct buttons', async () => {
    apiResponse.data.runner_list = [
      {
        donation: '5',
        email: 'test@example.com',
        firstname: 'Hans',
        id: 1,
        lastname: 'Meyer',
        payment_confirmation_mail_sent: true,
        payment_status: true,
        reason_for_payment: 'LGR-YPKDM',
        running_level: 'sometimes',
        start_number: 6,
        starting_point: 'other',
        team: 'FC St. Pauli',
        tshirt_cost: '15',
        verification_code: 'ogVXRyN8GpMSXUNV3VSx1ZBoUYwK95Sa8u'
      },
      {
        donation: '33',
        email: 'test5@example.com',
        firstname: 'Testy',
        id: 5,
        lastname: 'McTest',
        payment_confirmation_mail_sent: false,
        payment_status: false,
        reason_for_payment: 'LGR-YPKDX',
        running_level: 'sometimes',
        start_number: 66,
        starting_point: 'other',
        team: 'FC St. Pauli II',
        tshirt_cost: '33',
        verification_code: 'ogVXRyN8GpMSXUNV3VSx1ZBoUYwK95Sa8x'
      }
    ];
    change_payment_status.mockResolvedValueOnce(null);
    fetchFilteredRunners.mockResolvedValue(apiResponse);
    await act(async () => render(<Admin />));

    const table = screen.getByRole('table');
    const firstRowCells = within(table).getAllByRole('row')[1].children;
    const secondRowCells = within(table).getAllByRole('row')[2].children;

    expect(firstRowCells[7]).toHaveTextContent('Bezahlt');
    expect(secondRowCells[7]).toHaveTextContent('Nicht bezahlt');

    expect(firstRowCells[8]).toHaveTextContent('Bearbeiten');
    expect(secondRowCells[8]).toHaveTextContent('Bearbeiten');

    await userEvent.click(screen.getAllByRole('button', { name: 'Bearbeiten' })[0]);
    expect(router.push).toHaveBeenCalledWith({
      pathname: '/admin/edit',
      query: { id: '1' }
    });
    await userEvent.click(screen.getByRole('button', { name: 'Bezahlt' }));
    expect(change_payment_status).toHaveBeenCalledWith('1', false);
  });

  test('filters are applied correctly', async () => {
    apiResponse.data.runner_list = [];
    fetchFilteredRunners.mockResolvedValue(apiResponse);
    await act(async () => render(<Admin />));

    await userEvent.click(screen.getByRole('radio', { name: 'E-mail' }));
    await userEvent.type(screen.getByRole('textbox', { name: 'Suchbegriff' }), 'example');
    await userEvent.click(screen.getByRole('button', { name: 'Suche starten' }));

    expect(fetchFilteredRunners).toHaveBeenCalledWith(1, 'email', 'example');

    await userEvent.click(screen.getByRole('radio', { name: 'Startnummer' }));
    await userEvent.clear(screen.getByRole('textbox', { name: 'Suchbegriff' }));
    await userEvent.type(screen.getByRole('textbox', { name: 'Suchbegriff' }), '111');
    await userEvent.click(screen.getByRole('button', { name: 'Suche starten' }));

    expect(fetchFilteredRunners).toHaveBeenCalledWith(1, 'start_number', '111');
  });

  describe('pagination works as intended', () => {
    beforeEach(async () => {
      apiResponse.data.stats_number = 200;
      fetchFilteredRunners.mockResolvedValue(apiResponse);
      await act(async () => render(<Admin />));
    });

    test('page forward button', async () => {
      await userEvent.click(screen.getByRole('button', { name: '➡️' }));
      expect(fetchFilteredRunners).toHaveBeenCalledWith(2, 'name', '');
    });

    test('goto page function and page backward button', async () => {
      const pageInputField = screen.getByRole('textbox', { name: 'Seitenzahl' });
      await userEvent.clear(pageInputField);
      await userEvent.type(pageInputField, '10');
      await userEvent.click(screen.getByRole('button', { name: 'Gehe zu Seite' }));

      expect(fetchFilteredRunners).toHaveBeenCalledWith(10, 'name', '');
      expect(screen.getByText('10/14'));

      await userEvent.click(screen.getByRole('button',{name: '⬅'}));
      expect(fetchFilteredRunners).toHaveBeenCalledWith(9,'name','');
    });
  });
});
