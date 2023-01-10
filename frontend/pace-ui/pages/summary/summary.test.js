import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import SummaryPage from '.';
import React, { createContext } from 'react';
import { useJoinFormContext } from '../../context/JoinFormContext';
import JoinFormProvider from '../../context/JoinFormContext';
import { JoinFormContext } from '../../context/JoinFormContext';

describe('testing the summary page', () => {

  describe('should test without T-shirt', () => {
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
        <JoinFormContext.Provider
          value={{
            joinFormData: joinFormValues,
            setJoinFormData: () => { }
          }}
        >
          <SummaryPage />
        </JoinFormContext.Provider>
      );
    });

    test('renders page', () => {
      expect(screen.getByText('Zusammenfassung'));
    });

    test('displays basic information without shipping', () => {
      expect(screen.getByText('Fname', { exact: false }));
      expect(screen.getByText('Lname', { exact: false }));
      expect(screen.getByText('Team 1', { exact: false }));
      expect(screen.getByText('email@example.com', { exact: false }));
      expect(screen.getByText('Startort: Hamburg'));
      expect(screen.getByText('Laufniveau: Ich laufe häufig und ambitioniert.'));
      expect(screen.getByText('Spendenbeitrag: 11€'));
      expect(screen.queryByText('T-Shirt-Kosten', { exact: false })).not.toBeInTheDocument();
      expect(screen.queryByText('T-SHIRT ANGABEN', { exact: false })).not.toBeInTheDocument();
      expect(screen.queryByText('LIEFERADRESSE', { exact: false })).not.toBeInTheDocument();

    });
  });
  describe('should test with t-shirt', () => {
    const joinFormValues = {
      firstname: 'Fname',
      lastname: 'Lname',
      team: 'Team 1',
      email: 'email@example.com',
      repeated_email: 'email@example.com',
      starting_point: 'other',
      running_level: 'rarely',
      donation: 5,
      tshirt_cost: 15,
      tshirt_toggle: true,
      tshirt_model: 'unisex',
      tshirt_size: 's',
      country: 'Deutschland',
      address_firstname: 'Testy',
      address_lastname: 'McTest',
      street_name: 'Test avenue',
      house_number: '3',
      address_extra: 'fifth floor',
      postal_code: 'G12345',
      city: 'Cologne',

    };

    beforeEach(() => {
      render(
        <JoinFormContext.Provider
          value={{
            joinFormData: joinFormValues,
            setJoinFormData: () => { }
          }}
        >
          <SummaryPage />
        </JoinFormContext.Provider>
      );
    });

    test('displays basic information', () => {
      expect(screen.getByText('Name: Fname Lname', { exact: true }));
      expect(screen.getByText('Startort: Woanders'));
      expect(screen.getByText('Laufniveau: Ich laufe selten.'));
    });

    test('displays t-shirt information', () => {
      expect(screen.queryByText('T-SHIRT ANGABEN', { exact: false })).toBeInTheDocument();
      expect(screen.getByText('Modell: Unisex'));
      expect(screen.getByText('Größe: S'));
    });

    test('displays shipping information', () => {
      expect(screen.getByText('LIEFERADRESSE'));
      expect(screen.getByText('Testy McTest'));
      expect(screen.getByText('Test avenue 3'));
      expect(screen.getByText('G12345 Cologne'));
      expect(screen.getByText('fifth floor'));
      expect(screen.getByText('Deutschland'));
    });

    test('displays payment information', () => {
      expect(screen.getByText('Spendenbeitrag: 5€'));
      expect(screen.getByText('T-Shirt-Kosten: 15€')).toBeInTheDocument();
      expect(screen.getByText('Versand: kostenlos (innerhalb Deutschland)'));
      expect(screen.getByText('Zu zahlen: 20€'));
    });
  });

});
