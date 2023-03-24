import { describe, expect, test } from '@jest/globals';
import { render, screen } from '@testing-library/react';

import React from 'react';
import ConfirmationPage from '../../pages/confirmation';
import { RunnerContext } from '../../context/RunnerContext';

describe('should test confirmation page', () => {
  const contextContent = {
    runner_id: '123',
    start_number: '125',
    donation: '33',
    tshirt_cost: '15',
    payment: 'LGR-LXKJI',
    email_provided: false,
    verification_code: 'verificationCode'
  };
  beforeEach(() => {
    render(
      <RunnerContext.Provider
        value={{
          infoResponseData: contextContent,
          setInfoResponseData: () => {}
        }}
      >
        <ConfirmationPage />
      </RunnerContext.Provider>
    );
  });

  test('static elements are rendered', () => {
    expect(screen.getByText('Fast geschafft!'));
    expect(
      screen.getByText(
        'Super! Du hast dich vorläufig für den Lauf angemeldet. Um die Registrierung abzuschließen, überweise bitte deinen Spendenbetrag an folgendes Konto:'
      )
    );
    expect(screen.getByText('FC St. Pauli Marathon'));
    expect(screen.getByText('Hamburger Volksbank'));
    expect(screen.getByText('IBAN: DE09 2019 0003 0019 4004 20'));
    expect(screen.getByText('BLZ: GENODEF1HH2'));
  });

  test('dynamic elements are rendered', () => {
    expect(screen.getByText('Verwendungszweck: LGR-LXKJI'));
    expect(screen.getByText('Betrag: 48€'));
    expect(screen.getByRole('link', { name: 'Meinen Anmeldestatus abrufen' })).toHaveAttribute(
      'href',
      '/status/?runner_id=123&start_number=125&verification_code=verificationCode'
    );
    expect(
      screen.queryByText('Wir haben dir zudem diese Bestätigung an deine hinterlegte E-Mail Adresse gesendet.')
    ).not.toBeInTheDocument();
  });
});
