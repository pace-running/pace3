import { describe, expect, test } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';

import Button from '.';

describe('tests button component', () => {
  test('basic button displays label', () => {
    render(<Button name='test-btn' label='test-label' type='button' />);
    expect(screen.getByRole('button', { name: 'test-label' }));
    expect(screen.getByLabelText('test-label'));
  });

  test('button is clickable', async () => {
    const callback = jest.fn();
    render(<Button name='test-btn' label='test-label' type='button' onClick={callback} />);
    await userEvent.click(screen.getByRole('button'));
    expect(callback).toHaveBeenCalled();
  });
});
