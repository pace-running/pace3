import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';
import router from 'next/router';
import Header from '.';

describe('test header component', () => {
  beforeEach(() => {
    render(<Header />);
  });

  test('St.Pauli logo is visible and clickable', async () => {
    const logo = screen.getByRole('img');
    expect(logo.alt).toBe('FC St. Pauli Logo');
    expect(screen.getByRole('link', { name: 'FC St. Pauli Logo [event_name]' })).toHaveAttribute('href', '/');
  });

  test('Link to home page is displayed', () => {
    const link = screen.getByRole('link', { name: 'FC St. Pauli Logo [event_name]' });
    expect(link).toHaveTextContent('[event_name]');
    expect(link).toHaveAttribute('href', '/');
  });

  test('Button linking to join page is displayed', async () => {
    router.push = jest.fn();
    const button = screen.getByRole('link', { name: 'Hier Anmelden' });
    expect(button).toHaveTextContent('Hier Anmelden');

    await userEvent.click(button);
    expect(router.push).toHaveBeenCalledWith('/join');
  });
});
