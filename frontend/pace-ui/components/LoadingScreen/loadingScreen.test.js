import React from 'react';
import { describe, expect, test, jest } from '@jest/globals';
import { render, screen, within } from '@testing-library/react';
import LoadingScreen from '.';

describe('testing if the loading screen works', () => {
  beforeEach(() => {
    render(<LoadingScreen />);
  });

  test('checking static text', () => {
    expect(screen.getByText('Seite lädt...'));
  });

  test('checking for loading icon', () => {
    expect(screen.getByRole('img', { name: 'Lade-Icon' }));
  });
});
