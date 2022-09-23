import type { NextPage } from "next";
import { useFormik } from "formik";

import Button from "../../components/Button";
import Checkbox from "../../components/Checkbox";
import Dropdown from "../../components/Dropdown";
import BaseLayout from "../../components/Layout/baseLayout";
import TextInput from "../../components/TextInput";
import {
  getSizeOptions,
  modelOptions,
  runningLevelOptions,
  startingOptions,
} from "./dropdownOptions";
import { JoinFormSchema, JoinFormValues } from "./joinFormSchema";
import { useJoinFormContext } from "../../context/JoinFormContext";
import router from "next/router";

const Join: NextPage = () => {
  const { setJoinFormData } = useJoinFormContext();

  const submitForm = (values: JoinFormValues) => {
    setJoinFormData(values);
    router.push("/summary");
  };

  const { handleChange, setFieldValue, values, handleSubmit, errors } =
    useFormik<JoinFormValues>({
      initialValues: { donation: 10 },
      validationSchema: JoinFormSchema,
      onSubmit: submitForm,
    });

  return (
    <BaseLayout pageTitle="Anmeldung">
      <form onSubmit={handleSubmit}>
        <div className="container" style={{ maxWidth: "800px" }}>
          <h1>Anmeldung</h1>
          <p>Mit * markierte Felder müssen ausgefüllt werden.</p>

          <TextInput
            type={"text"}
            value={values.firstname}
            onChange={handleChange}
            name={"firstname"}
            label={"Vorname (erscheint auf der Startnummer)"}
          />
          <TextInput
            type={"text"}
            value={values.lastname}
            onChange={handleChange}
            name={"lastname"}
            label={"Nachname"}
          />
          <TextInput
            type={"text"}
            value={values.team}
            onChange={handleChange}
            name={"team"}
            label={"Team Name (erscheint auf der Startnummer)"}
          />
          <TextInput
            type={"email"}
            onChange={handleChange}
            value={values.email}
            name={"email"}
            valid={!errors.email}
            errorMessage={errors.email}
            label={"Email"}
          />
          <TextInput
            value={values.repeated_email}
            onChange={handleChange}
            type={"email"}
            name={"repeated_email"}
            label={"Email wiederholen"}
            valid={!errors.repeated_email}
            errorMessage={errors.repeated_email}
          />
          <Dropdown
            name={"starting_point"}
            label={"Von wo wirst du laufen? *"}
            options={startingOptions}
            selected={""}
            onChange={handleChange}
            valid={!errors.starting_point}
            errorMessage={errors.starting_point}
          />
          <Dropdown
            name={"running_level"}
            label={"Wie schätzt du dein Laufniveau ein? *"}
            options={runningLevelOptions}
            onChange={handleChange}
            valid={!errors.running_level}
            errorMessage={errors.running_level}
          />

          <TextInput
            type={"number"}
            name={"donation"}
            prependSignal="€"
            value={values.donation}
            valid={!errors.donation}
            errorMessage={errors.donation}
            onChange={handleChange}
            label={"Ich möchte spenden (mindestens 5€)"}
            helperLabel={"Wie möchtest du beitragen? *"}
          />

          <h2>Fan T-Shirt</h2>

          <Button
            name={"previewBtn"}
            label={"Vorschau"}
            type={"button"}
            styling={"preview-btn"}
          />
          <Button
            name={"sizesBtn"}
            label={"Größentabelle"}
            type={"button"}
            styling={"preview-btn"}
          />

          <Checkbox
            name={"tshirt_toggle"}
            check={values.tshirt_toggle}
            label={"Ich möchte ein T-Shirt"}
            role="switch"
            onChange={() =>
              setFieldValue("tshirt_toggle", !values.tshirt_toggle)
            }
          />

          {values.tshirt_toggle && (
            <div>
              <Dropdown
                name={"tshirt_model"}
                label={"Modell"}
                options={modelOptions}
                selected={""}
                onChange={handleChange}
              />
              <Dropdown
                name={"tshirt_size"}
                label={"Größe"}
                options={getSizeOptions(values.tshirt_model)}
                selected={""}
              />

              <h3>Lieferanschrift</h3>
              <TextInput
                type={"text"}
                value={values.country}
                onChange={handleChange}
                name={"country"}
                label={"Land *"}
                default={"Deutschland"}
              />
              <TextInput
                value={values.address_firstname}
                onChange={handleChange}
                type={"text"}
                name={"address_firstname"}
                label={"Vorname *"}
              />
              <TextInput
                value={values.address_lastname}
                onChange={handleChange}
                type={"text"}
                name={"address_lastname"}
                label={"Nachname *"}
              />
              <TextInput
                type={"text"}
                value={values.street_name}
                onChange={handleChange}
                name={"street_name"}
                label={"Straße *"}
              />
              <TextInput
                type={"text"}
                value={values.house_number}
                onChange={handleChange}
                name={"house_number"}
                label={"Hausnummer *"}
              />
              <TextInput
                value={values.address_extra}
                onChange={handleChange}
                type={"text"}
                name={"address_extra"}
                label={"Adresszusatz"}
              />
              <TextInput
                type={"text"}
                value={values.postal_code}
                onChange={handleChange}
                name={"postal_code"}
                label={"PLZ *"}
              />
              <TextInput
                type={"text"}
                value={values.city}
                onChange={handleChange}
                name={"city"}
                label={"Stadt *"}
              />
            </div>
          )}

          <Checkbox
            name={"tos_confirmed"}
            onChange={handleChange}
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
            check={values.tos_confirmed}
          />
          <Button
            name={"submitButton"}
            label={"Weiter"}
            type={"submit"}
            onSubmit={handleSubmit}
            disabled={!values.tos_confirmed}
            styling={"brownbg"}
          />
        </div>
      </form>
    </BaseLayout>
  );
};

export default Join;
