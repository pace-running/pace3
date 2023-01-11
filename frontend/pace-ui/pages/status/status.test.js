import { describe, expect, test } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';
import { act } from 'react-dom/test-utils'

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
        delivery_status: '',
    }
}

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
        delivery_status: 'In Bearbeitung',
    }
}

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
});

describe('test the status page with shipping info', () => {
    beforeEach(async () => {
        api.fetchRunnerDetails = jest.fn().mockReturnValue(response_with_shipping);
        await act(async () => render(<StatusPage />));
    });
    test('', () => { });
});