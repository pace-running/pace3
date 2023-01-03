import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import Join from '.';
// import userEvent from '@testing-library/user-event';

describe('testing of the registration page', () => {
  test('loads and displays join page', async () => {
    render(<Join />);
    expect(screen.getByText('Lauf gegen Rechts'));
    expect(screen.getAllByRole('heading')[0]).toHaveTextContent('Anmeldung');
    expect(screen.getAllByRole('heading')[1]).toHaveTextContent('Fan T-Shirt');
  });
});
