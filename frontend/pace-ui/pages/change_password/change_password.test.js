import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { act } from 'react-dom/test-utils';
import ChangePassword from '.';

describe ('change password page', () => {
    test('old password field is present', async () => {
        render(<ChangePassword />)
        await waitFor( () => {
            expect(screen.getByLabelText('Altes Passwort')).toBeInTheDocument();
        });
    });
    test('new password field is present', async () => {
        render(<ChangePassword />)
        await waitFor( () => {
            expect(screen.getByLabelText('Neues Passwort')).toBeInTheDocument();
        });
    });
    test('new password repeat field is present', async () => {
        render(<ChangePassword />)
        await waitFor( () => {
            expect(screen.getByLabelText('Neues Passwort wiederholen')).toBeInTheDocument();
        });
    });
});
