import type { NextPage } from "next";
import { useState } from "react";
import Checkbox from "../../components/Checkbox";
import Dropdown from "../../components/Dropdown";
import BaseLayout from "../../components/Layout/baseLayout";
import TextInput from "../../components/TextInput";

const startingOptions = [{label: "in Hamburg bei der Alster vor Ort", value: "hamburg"},{label: "woanders", value: "other"}];
const runningLevelOptions = [{label: "Ich laufe selten", value: "rarely"},{label: "Ich laufe gelegentlich bis regelmäßig", value: "sometimes"},{label: "Ich laufe häufig und ambitioniert", value: "often"}];

const Join: NextPage = () => {
    const [tshirt_toggle,setTshirtToggle] = useState(false);
    const toggleTshirtChangeHandler = () => {
        setTshirtToggle(!tshirt_toggle);
    }
  return (
    <BaseLayout pageTitle="Anmeldung">
        <div className="container">
            <h1>Anmeldung</h1>
            <p>Mit * markierte Felder müssen ausgefüllt werden.</p>

            <TextInput type={"text"} name={"firstname"} label={"Vorname (erscheint auf der Startnummer)"} />
            <TextInput type={"text"} name={"lastname"} label={"Nachname"} />
            <TextInput type={"text"} name={"team"} label={"Team Name (erscheint auf der Startnummer)"} />
            <TextInput type={"email"} name={"email"} label={"Email"} />
            <TextInput type={"email"} name={"repeat"} label={"Email wiederholen"} />
            <Dropdown name={"starting_point"} label={"Von wo wirst du laufen?"} options={startingOptions} selected={""}/>
            <Dropdown name={"running_level"} label={"Wie schätzt du dein Laufniveau ein?"} options={runningLevelOptions} selected={""}/>
            <TextInput type={"number"} name={"donation"} label={'Ich möchte spenden (mindestens 5€)'} helperLabel={"Wie möchtest du beitragen?"} />

            <h2>Fan T-Shirt</h2>
            
            <Checkbox name={"tshirt_toggle"} check={tshirt_toggle} label={"Ich möchte ein T-Shirt"} role="switch" onChange={toggleTshirtChangeHandler} />
            
        </div>
    </BaseLayout>
  );
};

export default Join;
