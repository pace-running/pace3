import React from 'react';
import Button from '../components/Button';

interface Props {
  statusContent: StatusResponseData | undefined;
}

const StatusContent: React.FC<Props> = props => {
  return (
    <div className='container' style={{ maxWidth: '800px', textAlign: 'center' }}>
      <h1>Deine Anmeldung</h1>
      <p>Hier kannst du den aktuellen Stand der Bearbeitung einsehen. </p>
      <b>
        Wir empfehlen dir diese Seite als Lesezeichen abzuspeichern, damit du den Status später jederzeit wieder abrufen
        kannst. Dies ist besonders wichtig, falls du keine Email hinterlegt hast.{' '}
      </b>
      <div>
        <h2 style={{fontSize: '1.75rem'}}>Deine Startnummer</h2>
        <p style={{ color: '#795548', fontSize: '36px' }}>{props.statusContent?.start_number}</p>
        <Button
          name={'downloadStartNumberBtn'}
          label={'Startnummer herunterladen'}
          type={'button'}
          styling={'brownbg'}
          onClick={() => {}}
        />
      </div>
      <div
        style={{
          textAlign: 'center',
          border: '3px solid grey',
          margin: '30px',
          padding: '20px'
        }}
      >
        <div style={{ textAlign: 'left', display: 'inline-block' }}>
          <h2>ZAHLUNG</h2>
          <h3>Spendenbetrag: {props.statusContent?.donation}€</h3>
          {!props.statusContent?.is_paid && (
            <div
              style={{
                display: 'inline-block',
                textAlign: 'left',
                paddingRight: '100px'
              }}
            >
              <p>Bitte auf folgendes Konto überweisen:</p>
              <p>
                FC St. Pauli Marathon
                <br />
                Hamburger Volksbank
                <br />
                IBAN: DE09 2019 0003 0019 4004 20
                <br />
                BLZ: GENODEF1HH2
                <br />
                Verwendungszweck: {props.statusContent?.payment}
              </p>
            </div>
          )}
          <div
            style={{
              display: 'inline-block',
              textAlign: 'left',
              verticalAlign: 'top'
            }}
          >
            <h3 style={{ paddingTop: '0' }}>STATUS</h3>
            <p>{props.statusContent?.is_paid ? 'Schon bezahlt' : 'Ausstehend'}</p>
          </div>
        </div>
      </div>
      {props.statusContent?.is_tshirt_booked && (
        <div
          style={{
            textAlign: 'center',
            border: '3px solid grey',
            margin: '30px',
            padding: '20px'
          }}
        >
          <div style={{ textAlign: 'left', display: 'inline-block' }}>
            <h2>T-SHIRT</h2>
            <div
              style={{
                display: 'inline-block',
                textAlign: 'left',
                paddingRight: '140px'
              }}
            >
              <p>
                Modell: {props.statusContent.tshirt_model === 'unisex' ? 'Unisex' : 'Tailliert'}
                <br />
                Größe: <span style={{ textTransform: 'uppercase' }}>{props.statusContent.tshirt_size}</span>
              </p>
              <p>
                {props.statusContent.address_firstname} {props.statusContent.address_lastname}
                <br />
                {props.statusContent.street_name} {props.statusContent.house_number}
                {props.statusContent.address_extra ? <br /> : <></>}
                {props.statusContent.address_extra}
                <br />
                {props.statusContent.postal_code} {props.statusContent.city}
                <br />
                {props.statusContent.country}
              </p>
            </div>
            <div
              style={{
                display: 'inline-block',
                textAlign: 'left',
                verticalAlign: 'top'
              }}
            >
              <h3 style={{ paddingTop: '0' }}>STATUS</h3>
              <p>{props.statusContent.delivery_status}</p>
            </div>
          </div>
        </div>
      )}
      {props.statusContent?.is_paid && (
        <div>
          <p>Vielen Dank für deine Anmeldung. Wir wünschen dir viel Spaß!</p>
          <p>Das Lauf-gegen-Rechts Team</p>
        </div>
      )}
    </div>
  );
};

export default StatusContent;
