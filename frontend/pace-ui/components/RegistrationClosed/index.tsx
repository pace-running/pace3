import React from 'react';
import { getThemeVar } from '../../utility/theme';
const RegistrationClosed: React.FC = () => {
  const message = getThemeVar('closed_registration_message');
  return (
    <div style={{ textAlign: 'center' }}>
      <h1>Anmeldung geschlossen!</h1>
      <p>{message}</p>
    </div>
  );
};

export default RegistrationClosed;
