import type { NextPage } from "next";
import { useState } from "react";
import Button from "../../components/Button";
import Checkbox from "../../components/Checkbox";
import Dropdown from "../../components/Dropdown";
import BaseLayout from "../../components/Layout/baseLayout";
import TextInput from "../../components/TextInput";

const startingOptions = [{label: "in Hamburg bei der Alster vor Ort", value: "hamburg"},{label: "woanders", value: "other"}];
const runningLevelOptions = [{label: "Ich laufe selten", value: "rarely"},{label: "Ich laufe gelegentlich bis regelmäßig", value: "sometimes"},{label: "Ich laufe häufig und ambitioniert", value: "often"}];
const modelOptions = [{label: "Unisex", value: "unisex"},{label: "Tailliert", value: "slimfit"}];
const sizeOptions = [{label: "S", value: "s"},{label: "M", value: "m"},{label: "L", value: "l"},{label: "XL", value: "xl"}];

const Join: NextPage = () => {
    const [tshirt_toggle,setTshirtToggle] = useState(false);
    const [TOSConfirmed,setTOSConfirmed] = useState(false);

    const toggleTshirtChangeHandler = () => {
        setTshirtToggle(!tshirt_toggle);
    };

    const modelOptionsChangeHandler = (event: any) =>{
        if (event.target.value==='unisex'){
            sizeOptions.push({label: "XXL", value: "xxl"});
        } else {
            if(sizeOptions.length === 5) sizeOptions.pop();
        }
    };

    const confirmTOS = () => {
        setTOSConfirmed(!TOSConfirmed);
    }

    const submitForm = () => {
        
    }
    
  return (
    <BaseLayout pageTitle="Anmeldung">
      <div className="container">
        <h1>Anmeldung</h1>
        <p>Mit * markierte Felder müssen ausgefüllt werden.</p>

        <TextInput
          type={"text"}
          name={"firstname"}
          label={"Vorname (erscheint auf der Startnummer)"}
        />
        <TextInput type={"text"} name={"lastname"} label={"Nachname"} />
        <TextInput
          type={"text"}
          name={"team"}
          label={"Team Name (erscheint auf der Startnummer)"}
        />
        <TextInput type={"email"} name={"email"} label={"Email"} />
        <TextInput type={"email"} name={"repeat"} label={"Email wiederholen"} />
        <Dropdown
          name={"starting_point"}
          label={"Von wo wirst du laufen?"}
          options={startingOptions}
          selected={""}
        />
        <Dropdown
          name={"running_level"}
          label={"Wie schätzt du dein Laufniveau ein?"}
          options={runningLevelOptions}
          selected={""}
        />
        <TextInput
          type={"number"}
          name={"donation"}
          label={"Ich möchte spenden (mindestens 5€)"}
          helperLabel={"Wie möchtest du beitragen?"}
        />

        <h2>Fan T-Shirt</h2>

        <Checkbox
          name={"tshirt_toggle"}
          check={tshirt_toggle}
          label={"Ich möchte ein T-Shirt"}
          role="switch"
          onChange={toggleTshirtChangeHandler}
        />

        {tshirt_toggle && (
          <div>
            <Dropdown
              name={"tshirt_model"}
              label={"Modell"}
              options={modelOptions}
              selected={""}
              onChange={modelOptionsChangeHandler}
            />
            <Dropdown
              name={"tshirt_size"}
              label={"Größe"}
              options={sizeOptions}
              selected={""}
            />

            <h3>Lieferanschrift</h3>
            <TextInput
              type={"text"}
              name={"country"}
              label={"Land"}
              default={"Deutschland"}
            />
            <TextInput
              type={"text"}
              name={"address_firstname"}
              label={"Vorname"}
            />
            <TextInput
              type={"text"}
              name={"address_lastname"}
              label={"Nachname"}
            />
            <TextInput type={"text"} name={"street_name"} label={"Straße"} />
            <TextInput
              type={"text"}
              name={"house_number"}
              label={"Hausnummer"}
            />
            <TextInput
              type={"text"}
              name={"address_extra"}
              label={"Adresszusatz"}
            />
            <TextInput type={"text"} name={"postal_code"} label={"PLZ"} />
            <TextInput type={"text"} name={"city"} label={"Stadt"} />
          </div>
        )}

        <Checkbox
          name={"confirm"}
          onChange={confirmTOS}
          label={""}
          rest={
            <span>
              Mir ist bewusst, dass die Datenverarbeitung entsprechend der{" "}
              <a className="link" href="/privacy_notice" target="_blank">
                Datenschutzbestimmungen
              </a>{" "}
              der Website lauf-gegen-rechts.de erfolgt. Ich weiß, dass meine
              Einwilligung bezüglich der Verarbeitung meiner Daten (Vorname,
              Nachname, E-Mail, Teamname freiwillig ist und ich sie jederzeit
              widerrufen kann.
            </span>
          }
          check={TOSConfirmed}
        />
        <Button name={"submitButton"} label={"Weiter"} type={"submit"} onSubmit={submitForm} disabled={!TOSConfirmed}/>
      </div>
    </BaseLayout>
  );
};

export default Join;
