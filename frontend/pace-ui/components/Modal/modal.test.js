import { describe, expect, test } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';

import Modal from '.';

describe('test modal window component', () => {
  const closemodal = jest.fn();

  test('open modal shows its contents', () => {
    render(<Modal name='test-modal' children={<div>Hello</div>} onClose={() => {}} open={true} />);
    expect(screen.getByText('Hello'));
  });

  test('closed modal does not show its contents', () => {
    render(<Modal name='test-modal' children={<div>Hello</div>} onClose={() => {}} open={false} />);
    expect(screen.queryByText('Hello')).not.toBeInTheDocument();
  });

  test('open modal can be closed', async () => {
    render(<Modal name='test-modal' children={<div>Hello</div>} onClose={closemodal} open={true} />);
    await userEvent.click(screen.getByRole('button', { name: 'Close' }));
    await userEvent.click(document.querySelector('.overlay'));

    expect(closemodal).toHaveBeenCalledTimes(2);
  });
});
