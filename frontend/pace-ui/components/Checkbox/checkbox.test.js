import { describe, expect, test } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';

import Checkbox from '.';

describe('test checkbox', () => {
  test('checkbox can be checked and unchecked', async () => {
    let checkboxStatus = false;
    const changeCheckBoxStatus = () => {
      checkboxStatus = !checkboxStatus;
    };
    const { rerender } = render(
      <Checkbox name='test-checkbox' label='checkbox-label' check={checkboxStatus} onChange={changeCheckBoxStatus} />
    );
    const checkbox = screen.getByRole('checkbox');
    expect(checkbox).not.toBeChecked();
    expect(screen.getByLabelText('checkbox-label')).toEqual(checkbox);

    await userEvent.click(checkbox);
    rerender(
      <Checkbox name='test-checkbox' label='checkbox-label' check={checkboxStatus} onChange={changeCheckBoxStatus} />
    );

    expect(checkboxStatus).toBe(true);
    await waitFor(() => {
      expect(checkbox).toBeChecked();
    });

    await userEvent.click(checkbox);
    rerender(
      <Checkbox name='test-checkbox' label='checkbox-label' check={checkboxStatus} onChange={changeCheckBoxStatus} />
    );

    await waitFor(() => {
      expect(checkbox).not.toBeChecked();
    });
    expect(checkboxStatus).toBe(false);
  });

  test('switch checkbox can be checked and unchecked', async () => {
    let checkboxStatus = false;
    const changeCheckBoxStatus = () => {
      checkboxStatus = !checkboxStatus;
    };
    const { rerender } = render(
      <Checkbox
        name='test-checkbox'
        role='switch'
        label='checkbox-label'
        check={checkboxStatus}
        onChange={changeCheckBoxStatus}
      />
    );
    const toggle = screen.getByRole('switch');
    expect(toggle).not.toBeChecked();
    expect(screen.getByLabelText('checkbox-label')).toEqual(toggle);

    await userEvent.click(toggle);
    rerender(
      <Checkbox
        name='test-checkbox'
        role='switch'
        label='checkbox-label'
        check={checkboxStatus}
        onChange={changeCheckBoxStatus}
      />
    );

    expect(checkboxStatus).toBe(true);
    await waitFor(() => {
      expect(toggle).toBeChecked();
    });

    await userEvent.click(toggle);
    rerender(
      <Checkbox
        name='test-checkbox'
        role='switch'
        label='checkbox-label'
        check={checkboxStatus}
        onChange={changeCheckBoxStatus}
      />
    );

    await waitFor(() => {
      expect(toggle).not.toBeChecked();
    });
    expect(checkboxStatus).toBe(false);
  });
});
