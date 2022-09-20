import type { NextPage } from "next";
import Dropdown from "../../components/Dropdown";
import BaseLayout from "../../components/Layout/baseLayout";
import TextInput from "../../components/TextInput";

const startingOptions = [{label: "in Hamburg bei der Alster vor Ort", value: "hamburg"},{label: "woanders", value: "other"}];
const runningLevelOptions = [{label: "Ich laufe selten", value: "rarely"},{label: "Ich laufe gelegentlich bis regelmäßig", value: "sometimes"},{label: "Ich laufe häufig und ambitioniert", value: "often"}];

const Join: NextPage = () => {
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
        </div>
    </BaseLayout>
  );
};

export default Join;
