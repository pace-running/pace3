import React from "react";

interface Props {
    donation: string;
    emailProvided: boolean;
}

const RegistrationConfirmation: React.FC<Props> = (props) => {
    return (
        <div
            className="container"
            style={{maxWidth: "800px", textAlign: "center"}}
        >
            <h1>Fast geschafft!</h1>
            <p>
                Super! Du hast dich vorläufig für den Lauf angemeldet. Um die Registrierung abzuschließen, überweise
                bitte deinen Spendenbetrag an folgendes Konto:
            </p>
            <div
                style={{
                    textAlign: "center",
                    border: "3px solid grey",
                    margin: "30px",
                    padding: "20px",
                }}
            >
                <div
                    style={{
                        display: "inline-block",
                        textAlign: "left",
                    }}
                >
                    <h2>Spendenbetrag: {props.donation}€</h2>
                    <p>FC St. Pauli Marathon</p>
                    <div
                        style={{
                            marginBottom: "0",
                            paddingTop: "0",
                            margin: "0px",
                        }}
                    >
                        <p>Hamburger Volksbank</p>
                        <p>IBAN: DE09 2019 0003 0019 4004 20</p>
                        <p>BLZ: GENODEF1HH2</p>
                        <p>Verwendungszweck: LGR-HUMKD</p>
                    </div>
                </div>
            </div>
            <p>Unter folgenden Link kannst du jederzeit den aktuellen Stand der Bearbeitung einsehen.</p>
            <p><a href="/status" style={{color: "red"}}>Meinen Anmeldestatus abrufen</a></p>
            {props.emailProvided && (
                <p>Wir haben dir zudem diese Bestätigung an deine hinterlegte E-Mail Adresse gesendet.</p>)}
        </div>
    );
}

export default RegistrationConfirmation;