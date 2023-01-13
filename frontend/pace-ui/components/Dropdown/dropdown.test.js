import { describe, expect, test } from '@jest/globals';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import React from 'react';

import Dropdown from '.';

describe('test dropdown menu', () => {
  const dropdownoptions = [
    { label: 'label1', value: 'value1' },
    { label: 'label2', value: 'value2' },
    { label: 'label3', value: 'value3' }
  ];
  beforeEach(() => {
    render(<Dropdown name='test-dropdown' label='dropdown-label' options={dropdownoptions} />);
  });

  test('dropdown menu has correct label', () => {
    expect(screen.getByLabelText('dropdown-label')).toEqual(screen.getByRole('combobox'));
  });

  test('says "bitte auswählen" by default', () => {
    expect(screen.getByText('Bitte auswählen'));
  });

  test('has correct options', () => {
    const dropdown = screen.getByRole('combobox');
    expect(dropdown.children[1]).toHaveTextContent('label1');
    expect(dropdown.children[2]).toHaveTextContent('label2');
    expect(dropdown.children[3]).toHaveTextContent('label3');
  });

  test('dropdown is interactive', async () => {
    const dropdown = screen.getByRole('combobox');
    expect(dropdown.value).toBe('Bitte auswählen');

    for (const option of dropdownoptions) {
      await userEvent.selectOptions(dropdown, option.value);
      expect(dropdown.value).toBe(option.value);
    }
  });
});
