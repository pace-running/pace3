import { describe, expect, test } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';
import { act } from 'react-dom/test-utils';

import StatusPage from '.';
import * as router from 'next/router';
import * as api from '../../apis/api';

const response = {
  status: 200,
  data: {
    runner_id: '101',
    start_number: '221',
    donation: '15',
    tshirt_cost: '0',
    payment: 'LGR-ASDFG',
    is_paid: false,

    is_tshirt_booked: false,
    tshirt_model: '',
    tshirt_size: '',
    country: '',
    address_firstname: '',
    address_lastname: '',
    street_name: '',
    house_number: '',
    address_extra: '',
    postal_code: '',
    city: '',
    delivery_status: ''
  }
};

const response_with_shipping = {
  status: 200,
  data: {
    runner_id: '101',
    start_number: '221',
    donation: '15',
    tshirt_cost: '17',
    payment: 'LGR-ASDFG',
    is_paid: true,

    is_tshirt_booked: true,
    tshirt_model: 'unisex',
    tshirt_size: 'm',
    country: 'Belgien',
    address_firstname: 'Testy',
    address_lastname: 'McTest',
    street_name: 'Testing blv',
    house_number: '77',
    address_extra: '',
    postal_code: '23569',
    city: 'Antwerp',
    delivery_status: 'In Bearbeitung'
  }
};

router.useRouter = jest.fn().mockReturnValue({
  query: {
    runner_id: 'runner_id',
    verification_code: 'verification_code'
  }
});

describe('test the status page without shipping info', () => {
  beforeEach(async () => {
    api.fetchRunnerDetails = jest.fn().mockReturnValue(response);
    await act(async () => render(<StatusPage />));
  });

  test('renders with proper mocking', () => {
    expect(jest.isMockFunction(api.fetchRunnerDetails)).toBeTruthy();
    expect(api.fetchRunnerDetails).toHaveBeenCalledWith('runner_id', 'verification_code');
    expect(screen.getByText('Deine Anmeldung'));
  });

  test('checking static components', () => {
    expect(screen.getByText('Hier kannst du den aktuellen Stand der Bearbeitung einsehen.'));
    expect(screen.getByText('Wir empfehlen dir diese Seite als Lesezeichen abzuspeichern,', { exact: false }));
    expect(screen.getByRole('button', { name: 'Startnummer herunterladen' }));
    expect(screen.getByText('ZAHLUNG'));
    expect(screen.getByText('FC St. Pauli Marathon', { exact: false }));
    expect(screen.getByText('Hamburger Volksbank', { exact: false }));
    expect(screen.getByText('IBAN: DE09 2019 0003 0019 4004 20', { exact: false }));
    expect(screen.getByText('BLZ: GENODEF1HH2', { exact: false }));
    expect(screen.getByText('STATUS'));
  });

  test('checking dynamic components', () => {
    expect(screen.getByText('221'));
    expect(screen.getByText('Betrag: 15€'));
    expect(screen.getByText('Verwendungszweck: LGR-ASDFG', { exact: false }));
    expect(screen.getByText('Ausstehend'));
    expect(screen.queryByText('T-SHIRT')).not.toBeInTheDocument();
  });
});

describe('test the status page with shipping info', () => {
  beforeEach(async () => {
    api.fetchRunnerDetails = jest.fn().mockReturnValue(response_with_shipping);
    await act(async () => render(<StatusPage />));
  });
  test('checking t-shirt fields', () => {
    expect(screen.getByRole('heading', { name: 'T-SHIRT' }));
    expect(screen.getByText('Unisex', { exact: false }));
    expect(screen.getByText('Testy McTest', { exact: false }));
    expect(screen.getByText('Testing blv 77', { exact: false }));
    expect(screen.getByText('23569 Antwerp', { exact: false }));
    expect(screen.getByText('Belgien', { exact: false }));
    expect(screen.getAllByText('STATUS')).toHaveLength(2);
    expect(screen.getByText('In Bearbeitung'));
  });
});
