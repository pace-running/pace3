import { describe, expect, test } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';
import router from 'next/router';
import Header from '.';

describe('test header component', () => {
  beforeEach(() => {
    render(<Header />);
  });

  test('St.Pauli logo is visible', () => {
    expect(screen.getByRole('img').alt).toBe('FC St. Pauli Logo');
  });

  test('Link to home page is displayed', () => {
    const link = screen.getByRole('link');
    expect(link).toHaveTextContent('Lauf gegen Rechts');
    expect(link).toHaveAttribute('href', '/');
  });

  test('Button linking to join page is displayed', async () => {
    router.push = jest.fn();
    const button = screen.getByRole('button');
    expect(button).toHaveTextContent('Anmelden');

    await userEvent.click(button);
    expect(router.push).toHaveBeenCalledWith('/join');
  });
});
