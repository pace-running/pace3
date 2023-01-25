import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { act } from 'react-dom/test-utils';

import { changePaymentStatus, editRunner, getFullRunner } from '../../apis/api';
import router from 'next/router';
import Edit from './edit';

jest.mock('next/router', () => ({
  useRouter: jest.fn(),
  push: jest.fn()
}));

jest.mock('../../apis/api', () => ({
  changePaymentStatus: jest.fn(),
  editRunner: jest.fn(),
  getFullRunner: jest.fn()
}));


describe('test edit page', () => {
  afterEach(() => {
    jest.clearAllMocks();
  });

  afterAll(() => {
    jest.restoreAllMocks();
  });

  const response = {
    status: 200,
    data: {
      id: 5,
      firstname: 'Testy',
      lastname: 'McTest',
      team: 'FC St. Pauli II',
      email: 'test5@example.com',
      starting_point: 'other',
      running_level: 'sometimes',
      donation: '33',

      is_tshirt_booked: true,
      tshirt_model: 'unisex',
      tshirt_size: 'l',
      country: 'Deutschland',
      address_firstname: 'Testy',
      address_lastname: 'McTest',
      street_name: 'Test street',
      house_number: '42',
      address_extra: '',
      postal_code: '12345',
      city: 'testing city',

      start_number: 66,
      verification_code: 'ogVXRyN8GpMSXUNV3VSx1ZBoUYwK95Sa8x',
      reason_for_payment: 'LGR-YPKDX',
      payment_status: false,
      delivery_status: 'In Bearbeitung',
      payment_confirmation_mail_sent: false
      // tshirt_cost: '33',
    }
  };

  test('runner information is displayed correctly', async () => {
    router.useRouter.mockReturnValue({ query: { id: 5 } });
    getFullRunner.mockReturnValue(response);
    await act(async () => {
      render(<Edit />);
    });
    expect(getFullRunner).toHaveBeenCalledWith(5);

    expect(screen.getByRole('textbox',{name: 'Vorname (erscheint auf der Startnummer)'})).toHaveAttribute('placeholder','Testy');
    expect(screen.getByRole('textbox',{name: 'Nachname'})).toHaveAttribute('placeholder','McTest');
  });
});
