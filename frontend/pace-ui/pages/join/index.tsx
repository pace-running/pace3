import type { NextPage } from 'next';
import { useFormik } from 'formik';

import Button from '../../components/Button';
import Checkbox from '../../components/Checkbox';
import Dropdown from '../../components/Dropdown';
import BaseLayout from '../../components/Layout/baseLayout';
import TextInput from '../../components/TextInput';
import SizeTable from '../../components/SizeTable';
import Modal from '../../components/Modal';
import {
  euCountryOptions,
  getSizeOptions,
  modelOptions,
  regionOptions,
  runningLevelOptions,
  startingOptions
} from '../../utility/dropdownOptions';
import { JoinFormSchema, JoinFormValues } from '../../utility/joinFormSchema';
import { useJoinFormContext } from '../../context/JoinFormContext';
import router from 'next/router';
import { useEffect, useState } from 'react';
import { getThemeVar } from '../../utility/theme';
import RegistrationClosed from '../../components/RegistrationClosed';

const Join: NextPage = () => {
  const { joinFormData, setJoinFormData } = useJoinFormContext();

  const [showPreviewModal, setShowPreviewModal] = useState(false);
  const [showSizesModal, setShowSizesModal] = useState(false);
  const [shippingRegion, setShippingRegion] = useState('');

  const isRegistrationOpen = getThemeVar('is_registration_open');
  const tshirtsEnabled = getThemeVar('enable_tshirts');

  const submitForm = (values: JoinFormValues) => {
    // console.log(`Tshirt_cost when submitting join: ${values.tshirt_cost}`);
    setJoinFormData(values);
    router.push('/summary');
  };

  const { handleChange, setFieldValue, values, handleSubmit, errors, isValid } = useFormik<JoinFormValues>({
    initialValues: {
      tshirt_cost: 0,
      tshirt_toggle: false,
      tos_confirmed: false,
      firstname: '',
      lastname: '',
      team: '',
      email: '',
      repeated_email: '',
      starting_point: '',
      running_level: '',
      donation: 10,
      bsv_participant: false,
      tshirt_model: '',
      tshirt_size: '',
      country: '',
      address_firstname: '',
      address_lastname: '',
      street_name: '',
      house_number: '',
      address_extra: '',
      postal_code: '',
      city: ''
    },
    validationSchema: JoinFormSchema,
    onSubmit: submitForm
  });

  useEffect(() => {
    if (joinFormData) {
      for (const [key, val] of Object.entries(joinFormData)) {
        setFieldValue(key, val);
      }
      values.running_level = joinFormData?.running_level;
      values.starting_point = joinFormData?.starting_point;
      if (joinFormData.country === 'Deutschland') {
        setShippingRegion('de');
      } else if (
        Object.values(euCountryOptions)
          .map(obj => obj.label)
          .includes(joinFormData.country ?? '--')
      ) {
        setShippingRegion('eu');
      } else setShippingRegion('non-eu');
    }
  }, [joinFormData]);

  const update_tshirt_cost = (toggle: boolean, region: string) => {
    if (toggle) {
      if (region === 'de') setFieldValue('tshirt_cost', 15);
      if (region === 'eu') setFieldValue('tshirt_cost', 17);
      if (region === 'non-eu') setFieldValue('tshirt_cost', 20);
    } else setFieldValue('tshirt_cost', 0);
  };
  if (isRegistrationOpen === 'true') {
    return (
      <BaseLayout pageTitle='Anmeldung'>
        <form onSubmit={handleSubmit}>
          <div className='container' style={{ maxWidth: '800px' }}>
            <h1>Anmeldung</h1>
            <p>Mit * markierte Felder müssen ausgefüllt werden.</p>

            <TextInput
              type={'text'}
              value={values.firstname}
              onChange={handleChange}
              name={'firstname'}
              label={'Vorname (erscheint auf der Startnummer)'}
              valid={!errors.firstname}
              errorMessage={errors.firstname}
            />
            <TextInput
              type={'text'}
              value={values.lastname}
              onChange={handleChange}
              name={'lastname'}
              label={'Nachname'}
              valid={!errors.lastname}
              errorMessage={errors.lastname}
            />
            <TextInput
              type={'text'}
              value={values.team}
              onChange={handleChange}
              name={'team'}
              label={'Team Name (erscheint auf der Startnummer)'}
            />
            <Checkbox
              name={'bsv_participant'}
              label={'Wir starten als Betriebssport (BSV) Team'}
              check={values.bsv_participant}
              onChange={() => {
                setFieldValue('bsv_participant', !values.bsv_participant);
              }}
            />
            <TextInput
              type={'email'}
              onChange={handleChange}
              value={values.email}
              name={'email'}
              valid={!errors.email}
              errorMessage={errors.email}
              label={'Email'}
            />
            <TextInput
              value={values.repeated_email}
              onChange={handleChange}
              type={'email'}
              name={'repeated_email'}
              label={'Email wiederholen'}
              valid={!errors.repeated_email}
              errorMessage={errors.repeated_email}
            />
            <Dropdown
              name={'starting_point'}
              label={'Von wo wirst du laufen? *'}
              options={startingOptions}
              selected={''}
              onChange={handleChange}
              default={joinFormData?.starting_point}
              valid={!errors.starting_point}
              errorMessage={errors.starting_point}
            />
            <Dropdown
              name={'running_level'}
              label={'Wie schätzt du dein Laufniveau ein? *'}
              options={runningLevelOptions}
              onChange={handleChange}
              default={joinFormData?.running_level}
              valid={!errors.running_level}
              errorMessage={errors.running_level}
            />

            <TextInput
              type={'number'}
              name={'donation'}
              // prependSignal="€"
              value={values.donation}
              valid={!errors.donation}
              errorMessage={errors.donation}
              onChange={handleChange}
              label={'Ich möchte spenden (mindestens 5€)'}
              helperLabel={'Wie möchtest du beitragen? *'}
            />

            {tshirtsEnabled === 'true' && (
              <div>
                <h2>Fan T-Shirt</h2>

                <Button
                  name={'previewBtn'}
                  label={'Vorschau'}
                  type={'button'}
                  styling={'preview-btn'}
                  onClick={() => setShowPreviewModal(true)}
                />
                <Button
                  name={'sizesBtn'}
                  label={'Größentabelle'}
                  type={'button'}
                  styling={'preview-btn'}
                  onClick={() => setShowSizesModal(true)}
                />

                <Modal name={'previewModal'} open={showPreviewModal} onClose={() => setShowPreviewModal(false)}>
                  <div>
                    <h3>T-Shirt Vorschau</h3>
                    <img src='tshirt_preview.png' alt='T-shirt Preview' width={200} height={200}></img>
                  </div>
                </Modal>

                <Modal name={'sizesModal'} open={showSizesModal} onClose={() => setShowSizesModal(false)}>
                  <div>
                    <SizeTable />
                  </div>
                </Modal>

                <Checkbox
                  name={'tshirt_toggle'}
                  check={values.tshirt_toggle}
                  label={'Ich möchte ein T-Shirt (Kosten: 15€)'}
                  role='switch'
                  onChange={() => {
                    update_tshirt_cost(!values.tshirt_toggle, shippingRegion);
                    setFieldValue('tshirt_toggle', !values.tshirt_toggle);
                  }}
                />

                {values.tshirt_toggle && (
                  <div>
                    <Dropdown
                      name={'tshirt_model'}
                      label={'Modell'}
                      options={modelOptions}
                      selected={''}
                      onChange={handleChange}
                      default={values?.tshirt_model}
                      errorMessage={errors.tshirt_model}
                    />
                    <Dropdown
                      name={'tshirt_size'}
                      label={'Größe'}
                      options={getSizeOptions(values.tshirt_model)}
                      selected={''}
                      onChange={handleChange}
                      default={values?.tshirt_size}
                      errorMessage={errors.tshirt_size}
                    />

                    <h3>Lieferanschrift</h3>

                    <Dropdown
                      name={'region'}
                      label={'Region *'}
                      options={regionOptions}
                      selected={''}
                      errorMessage={shippingRegion ? '' : errors.country}
                      onChange={e => {
                        const value = (e.target as HTMLInputElement).value;
                        if (value === 'de') setFieldValue('country', 'Deutschland');
                        if (value === 'non-eu') setFieldValue('country', '');
                        setShippingRegion(value);
                        update_tshirt_cost(true, value);
                      }}
                      default={shippingRegion}
                    />

                    {shippingRegion === 'de' && (
                      <div className='mb-3'>
                        <label htmlFor={'static-country-de'} className='form-label'>
                          Land *
                        </label>
                        <div className='input-group'>
                          <input
                            id={'static-country-de'}
                            type={'text'}
                            value={'Deutschland'}
                            className='form-control'
                            name={'static-country-de'}
                            readOnly
                          />
                        </div>
                      </div>
                    )}

                    {shippingRegion === 'eu' && (
                      <Dropdown
                        name={'country-eu'}
                        label={'Land *'}
                        options={euCountryOptions}
                        selected={''}
                        onChange={e => {
                          setFieldValue('country', (e.target as HTMLInputElement).value);
                        }}
                        default={values?.country}
                        errorMessage={errors.country}
                        valid={!errors.country}
                      />
                    )}

                    {shippingRegion === 'non-eu' && (
                      <TextInput
                        value={values.country}
                        onChange={e => {
                          setFieldValue('country', (e.target as HTMLInputElement).value);
                        }}
                        type={'text'}
                        name={'country-non-eu'}
                        label={'Land *'}
                        valid={!errors.country}
                        errorMessage={errors.country}
                      />
                    )}

                    <TextInput
                      value={values.address_firstname}
                      onChange={handleChange}
                      type={'text'}
                      name={'address_firstname'}
                      label={'Vorname *'}
                      valid={!errors.address_firstname}
                      errorMessage={errors.address_firstname}
                    />
                    <TextInput
                      value={values.address_lastname}
                      onChange={handleChange}
                      type={'text'}
                      name={'address_lastname'}
                      label={'Nachname *'}
                      valid={!errors.address_lastname}
                      errorMessage={errors.address_lastname}
                    />
                    <TextInput
                      type={'text'}
                      value={values.street_name}
                      onChange={handleChange}
                      name={'street_name'}
                      label={'Straße *'}
                      valid={!errors.street_name}
                      errorMessage={errors.street_name}
                    />
                    <TextInput
                      type={'text'}
                      value={values.house_number}
                      onChange={handleChange}
                      name={'house_number'}
                      label={'Hausnummer *'}
                      valid={!errors.house_number}
                      errorMessage={errors.house_number}
                    />
                    <TextInput
                      value={values.address_extra}
                      onChange={handleChange}
                      type={'text'}
                      name={'address_extra'}
                      label={'Adresszusatz'}
                    />
                    <TextInput
                      type={'text'}
                      value={values.postal_code}
                      onChange={handleChange}
                      name={'postal_code'}
                      label={'PLZ *'}
                      valid={!errors.postal_code}
                      errorMessage={errors.postal_code}
                    />
                    <TextInput
                      type={'text'}
                      value={values.city}
                      onChange={handleChange}
                      name={'city'}
                      label={'Stadt *'}
                      valid={!errors.city}
                      errorMessage={errors.city}
                    />
                  </div>
                )}
              </div>
            )}

            <Checkbox
              name={'tos_confirmed'}
              onChange={handleChange}
              label={''}
              rest={
                <span>
                  Mir ist bewusst, dass die Datenverarbeitung entsprechend der{' '}
                  <a className='link' href='/privacy_notice' target='_blank'>
                    Datenschutzbestimmungen
                  </a>{' '}
                  der Website lauf-gegen-rechts.de erfolgt. Ich weiß, dass meine Einwilligung bezüglich der Verarbeitung
                  meiner Daten (Vorname, Nachname, E-Mail, Teamname) freiwillig ist und ich sie jederzeit widerrufen
                  kann.
                </span>
              }
              check={values.tos_confirmed}
            />
            <Button
              name={'submitButton'}
              label={'Weiter'}
              type={'submit'}
              onSubmit={handleSubmit}
              disabled={!(values.tos_confirmed && isValid)}
              styling={'brownbg'}
            />
          </div>
        </form>
      </BaseLayout>
    );
  } else {
    return (
      <BaseLayout pageTitle='Anmeldung geschlossen'>
        <RegistrationClosed />
      </BaseLayout>
    );
  }
};

export default Join;
