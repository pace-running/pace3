import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import SummaryPage from '.';
import React, { createContext } from 'react';
import { useJoinFormContext } from '../../context/JoinFormContext';
import JoinFormProvider from '../../context/JoinFormContext';

describe('testing the summary page', () => {
  const joinFormValues = {
    firstname: 'Fname',
    lastname: 'Lname',
    team: 'Team 1',
    email: 'email@example.com',
    repeated_email: 'email@example.com',
    starting_point: 'hamburg',
    running_level: 'often',
    donation: 11,
    tshirt_cost: 0,
    tshirt_toggle: false
  };

  beforeEach(() => {
    render(
      (() => {
        const JoinFormContext = createContext({
          joinFormData: joinFormValues,
          setJoinFormData: () => {}
        });

        return (
          <JoinFormContext.Provider
            value={{
              joinFormData: joinFormValues,
              setJoinFormData: () => {}
            }}
          >
            <SummaryPage />
          </JoinFormContext.Provider>
        );
      })()
    );
  });

  test('renders page', () => {
    expect(screen.getByText('Zusammenfassung'));
  });

  test('context loaded', () => {
    // expect(screen.getByText('Fname'));
  });
});
