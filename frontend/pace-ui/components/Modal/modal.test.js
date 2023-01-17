import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';

import Modal from '.';

describe('test modal window component', () => {
  const closemodal = jest.fn();

  test('open modal shows its contents', () => {
    render(
      <Modal name='test-modal' onClose={() => {}} open={true}>
        {<div>Hello</div>}
      </Modal>
    );
    expect(screen.getByText('Hello'));
  });

  test('closed modal does not show its contents', () => {
    render(
      <Modal name='test-modal' onClose={() => {}} open={false}>
        {<div>Hello</div>}
      </Modal>
    );
    expect(screen.queryByText('Hello')).not.toBeInTheDocument();
  });

  test('open modal can be closed', async () => {
    render(
      <Modal name='test-modal' onClose={closemodal} open={true}>
        {<div>Hello</div>}
      </Modal>
    );
    await userEvent.click(screen.getByRole('button', { name: 'Close' }));
    await userEvent.click(document.querySelector('.overlay'));

    expect(closemodal).toHaveBeenCalledTimes(2);
  });
});
