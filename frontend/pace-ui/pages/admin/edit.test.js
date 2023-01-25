import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { act } from 'react-dom/test-utils';

import { changePaymentStatus, editRunner, getFullRunner } from '../../apis/api';
import router from 'next/router';
import Edit from './edit';

jest.mock('next/router', () => ({
  useRouter: jest.fn(),
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
    router.useRouter.mockReturnValue({ query: { id: 5 }});
    getFullRunner.mockReturnValue(response);
    await act(async () => {
      render(<Edit />);
    });
    expect(getFullRunner).toHaveBeenCalledWith(5);

    expect(screen.getByRole('textbox', { name: 'Vorname (erscheint auf der Startnummer)' })).toHaveValue('Testy');
    expect(screen.getByRole('textbox', { name: 'Nachname' })).toHaveValue('McTest');
    expect(screen.getByRole('combobox', { name: 'Größe' })).toHaveValue('l');
  });

  test('changes are correctly sent to backend', async () => {
    const mockPush = jest.fn();
    router.useRouter.mockReturnValue({ query: { id: 5 }, push: mockPush });
    getFullRunner.mockReturnValue(response);
    editRunner.mockResolvedValue(null);
    await act(async () => {
      render(<Edit />);
    });
    await userEvent.type(screen.getByRole('spinbutton', { name: 'Ich möchte spenden (mindestens 5€)' }), '0');
    await userEvent.selectOptions(screen.getByRole('combobox', { name: 'Von wo wirst du laufen? *' }), 'other');
    await userEvent.click(screen.getByRole('switch', { name: 'Ich möchte ein T-Shirt' }));
    await userEvent.click(screen.getByRole('button', { name: 'Änderungen bestätigen' }));

    expect(mockPush).toHaveBeenCalledWith('/admin');
    expect(editRunner).toHaveBeenCalledWith(5, {
      address_extra: '',
      address_firstname: 'Testy',
      address_lastname: 'McTest',
      city: 'testing city',
      country: 'Deutschland',
      delivery_status: 'In Bearbeitung',
      donation: '330',
      email: 'test5@example.com',
      firstname: 'Testy',
      house_number: '42',
      is_tshirt_booked: false,
      lastname: 'McTest',
      payment_confirmation_mail_sent: false,
      payment_status: false,
      postal_code: '12345',
      reason_for_payment: 'LGR-YPKDX',
      runner_id: 5,
      running_level: 'sometimes',
      start_number: 66,
      starting_point: 'other',
      street_name: 'Test street',
      team: 'FC St. Pauli II',
      tshirt_model: 'unisex',
      tshirt_size: 'l',
      verification_code: 'ogVXRyN8GpMSXUNV3VSx1ZBoUYwK95Sa8x'
    });
  });

  test('payment status button',async ()=>{
    router.useRouter.mockReturnValue({ query: { id: 5 } });
    getFullRunner.mockReturnValue(response);
    changePaymentStatus.mockResolvedValue(null);
    await act(async () => {
      render(<Edit />);
    });
    await userEvent.click(screen.getByRole('button',{name: 'Nicht bezahlt'}));
    expect(changePaymentStatus).toHaveBeenCalledWith('5',true);
  });

  test('button back to admin page opens confirmation modal',async ()=>{
    router.useRouter.mockReturnValue({ query: { id: 5 } });
    getFullRunner.mockReturnValue(response);
    await act(async () => {
      render(<Edit />);
    });
    await userEvent.click(screen.getByRole('button',{name: 'Zurück zur Adminseite'}));
    expect(screen.getByText('Sind Sie sicher, dass sie den Bearbeitungsvorgang abbrechen und alle bisherigen Änderungen verwerfen möchten?'));
    expect(screen.getByRole('button', {name: 'Ja, zurück zur Adminseite'}));
    expect(screen.getByRole('button', {name: 'Nein, Bearbeitung fortsetzen'}));
  });

  test('continue button in confirmation modal closes modal', async ()=>{
    router.useRouter.mockReturnValue({ query: { id: 5 } });
    getFullRunner.mockReturnValue(response);
    await act(async () => {
      render(<Edit />);
    });
    await userEvent.click(screen.getByRole('button',{name: 'Zurück zur Adminseite'}));
    await userEvent.click(screen.getByRole('button', {name: 'Nein, Bearbeitung fortsetzen'}));
    expect(screen.queryByText('Sind Sie sicher, dass sie den Bearbeitungsvorgang abbrechen und alle bisherigen Änderungen verwerfen möchten?')).not.toBeInTheDocument();
  });

  test('back button in confirmation modal routes back to admin page',async ()=>{
    const mockPush = jest.fn();
    router.useRouter.mockReturnValue({ query: { id: 5 } , push: mockPush});
    getFullRunner.mockReturnValue(response);
    await act(async () => {
      render(<Edit />);
    });
    await userEvent.click(screen.getByRole('button',{name: 'Zurück zur Adminseite'}));
    await userEvent.click(screen.getByRole('button', {name: 'Ja, zurück zur Adminseite'}));
    expect(mockPush).toHaveBeenCalledWith('/admin');
  });
});
