import { useFormik } from 'formik';
import { NextPage } from 'next';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { change_payment_status, edit_runner, get_full_runner } from '../../apis/api';
import Button from '../../components/Button';
import Checkbox from '../../components/Checkbox';
import Dropdown from '../../components/Dropdown';
import TextInput from '../../components/TextInput';
import { getSizeOptions, modelOptions, runningLevelOptions, startingOptions } from '../../utility/dropdownOptions';
import { EditRunnerSchema, EditRunnerValues } from '../../utility/editRunnerSchema';

const Edit: NextPage = () => {
  const router = useRouter();
  const runner_id = router.query.id as string;
  const [isPageFound, setIsPageFound] = useState(false);

  const [runnerData, setRunnerData] = useState<fullRunnerData>();

  useEffect(() => {
    const fetchData = async () => {
      if (runner_id) {
        // Could use verification code for additional layer of security
        const response = await get_full_runner(runner_id);
        if (response.status === 200) {
          // set contents with response data
          setRunnerData(response.data);
          setIsPageFound(true);
          values.is_tshirt_booked = runnerData ? runnerData?.is_tshirt_booked : false;
          values.payment_status = runnerData ? runnerData?.payment_status : false;
          values.donation = runnerData ? +runnerData?.donation : 5;
          values.tshirt_model = runnerData ? runnerData.tshirt_model : 'unisex';
          values.tshirt_size = runnerData ? runnerData.tshirt_size : 'm';
        }
      }
    };
    fetchData();
  }, [runner_id, isPageFound]);

  const mapEditRunnerDataToFullRunnerData = (formData: EditRunnerValues) => ({
    runner_id: runner_id,
    firstname: formData.firstname ?? runnerData?.firstname ?? '',
    lastname: formData.lastname ?? runnerData?.lastname ?? '',
    team: formData.team ?? runnerData?.team ?? '',
    email: formData.email ?? runnerData?.email ?? '',
    starting_point: formData.starting_point ?? runnerData?.starting_point ?? '',
    running_level: formData.running_level ?? runnerData?.running_level ?? '',
    donation: formData.donation ? formData.donation.toString() : runnerData?.donation ?? '5',
    is_tshirt_booked: formData.is_tshirt_booked ?? false,
    tshirt_model: formData.tshirt_model ?? runnerData?.tshirt_model ?? '',
    tshirt_size: formData.tshirt_size ?? runnerData?.tshirt_size ?? '',
    country: formData.country ?? runnerData?.country ?? '',
    address_firstname: formData.address_firstname ?? runnerData?.address_firstname ?? '',
    address_lastname: formData.address_lastname ?? runnerData?.address_lastname ?? '',
    street_name: formData.street_name ?? runnerData?.street_name ?? '',
    house_number: formData.house_number ?? runnerData?.house_number ?? '',
    address_extra: formData.address_extra ?? runnerData?.address_extra ?? '',
    postal_code: formData.postal_code ?? runnerData?.postal_code ?? '',
    city: formData.city ?? runnerData?.city ?? '',
    start_number: formData.start_number ?? runnerData?.start_number ?? '',
    verification_code: formData.verification_code ?? runnerData?.verification_code ?? '',
    reason_for_payment: formData.reason_for_payment ?? runnerData?.reason_for_payment ?? '',
    payment_status: formData.payment_status ?? runnerData?.payment_status ?? '',
    delivery_status: formData.delivery_status ?? runnerData?.delivery_status ?? '',
    payment_confirmation_mail_sent: runnerData?.payment_confirmation_mail_sent ?? false
  });

  const submitForm = (values: EditRunnerValues) => {
    console.log('submitting...');
    edit_runner(runner_id, mapEditRunnerDataToFullRunnerData(values)).then(() => router.push('/admin'));
  };

  const { handleChange, setFieldValue, values, handleSubmit, errors } = useFormik<EditRunnerValues>({
    initialValues: {
      firstname: runnerData?.firstname,
      lastname: runnerData?.lastname,
      team: runnerData?.team,
      email: runnerData?.email,
      starting_point: runnerData?.starting_point,
      running_level: runnerData?.running_level,
      donation: runnerData ? +runnerData?.donation : 5,
      is_tshirt_booked: runnerData?.is_tshirt_booked ?? false,
      tshirt_model: runnerData?.tshirt_model,
      tshirt_size: runnerData?.tshirt_size,
      country: runnerData?.country,
      address_firstname: runnerData?.address_firstname,
      address_lastname: runnerData?.address_lastname,
      street_name: runnerData?.street_name,
      house_number: runnerData?.house_number,
      address_extra: runnerData?.address_extra,
      postal_code: runnerData?.postal_code,
      city: runnerData?.city,
      start_number: runnerData?.start_number,
      verification_code: runnerData?.verification_code,
      reason_for_payment: runnerData?.reason_for_payment,
      payment_status: runnerData?.payment_status,
      delivery_status: runnerData?.delivery_status
    },
    validationSchema: EditRunnerSchema,
    onSubmit: submitForm
  });

  return (
    <div>
      <h1>Edit Runner:</h1>
      {isPageFound && (
        <form onSubmit={handleSubmit}>
          <div className='container' style={{ maxWidth: '800px' }}>
            <TextInput
              type={'text'}
              value={values.firstname}
              onChange={handleChange}
              name={'firstname'}
              label={'Vorname (erscheint auf der Startnummer)'}
              placeholder={runnerData?.firstname}
              valid={!errors.firstname}
              errorMessage={errors.firstname}
            />
            <TextInput
              type={'text'}
              value={values.lastname}
              onChange={handleChange}
              name={'lastname'}
              label={'Nachname'}
              placeholder={runnerData?.lastname}
              valid={!errors.lastname}
              errorMessage={errors.lastname}
            />
            <TextInput
              type={'text'}
              value={values.team}
              onChange={handleChange}
              name={'team'}
              label={'Team Name (erscheint auf der Startnummer)'}
              placeholder={runnerData?.team}
            />
            <TextInput
              type={'email'}
              onChange={handleChange}
              value={values.email}
              name={'email'}
              valid={!errors.email}
              errorMessage={errors.email}
              label={'Email'}
              placeholder={runnerData?.email}
            />
            <Dropdown
              name={'starting_point'}
              label={'Von wo wirst du laufen? *'}
              options={startingOptions}
              onChange={handleChange}
              default={runnerData?.starting_point}
              valid={!errors.starting_point}
            />
            <Dropdown
              name={'running_level'}
              label={'Wie schätzt du dein Laufniveau ein? *'}
              options={runningLevelOptions}
              onChange={handleChange}
              default={runnerData?.running_level}
              valid={!errors.running_level}
            />
            <TextInput
              type={'number'}
              name={'donation'}
              // prependSignal="€"
              value={values.donation}
              valid={!errors.donation}
              onChange={handleChange}
              placeholder={runnerData?.donation}
              label={'Ich möchte spenden (mindestens 5€)'}
              helperLabel={'Wie möchtest du beitragen? *'}
            />
            <h2>Fan T-Shirt</h2>
            <Checkbox
              name={'tshirt_toggle'}
              check={values.is_tshirt_booked}
              label={'Ich möchte ein T-Shirt'}
              role='switch'
              onChange={() => setFieldValue('is_tshirt_booked', !values.is_tshirt_booked)}
            />
            <div
              style={{
                pointerEvents: values.is_tshirt_booked ? 'auto' : 'none',
                color: values.is_tshirt_booked ? 'black' : 'gray'
              }}
            >
              <Dropdown
                name={'tshirt_model'}
                label={'Modell'}
                options={modelOptions}
                default={runnerData?.tshirt_model}
                onChange={handleChange}
              />
              <Dropdown
                name={'tshirt_size'}
                label={'Größe'}
                options={getSizeOptions(values.tshirt_model)}
                default={runnerData?.tshirt_size}
                onChange={handleChange}
              />

              <h3>Lieferanschrift</h3>

              <TextInput
                value={values.country}
                onChange={handleChange}
                type={'text'}
                name={'country'}
                label={'Land *'}
                placeholder={runnerData?.country}
              />
              <TextInput
                value={values.address_firstname}
                onChange={handleChange}
                type={'text'}
                name={'address_firstname'}
                label={'Vorname *'}
                placeholder={runnerData?.address_firstname}
                valid={!errors.address_firstname}
                errorMessage={errors.address_firstname}
              />
              <TextInput
                value={values.address_lastname}
                onChange={handleChange}
                type={'text'}
                name={'address_lastname'}
                label={'Nachname *'}
                placeholder={runnerData?.address_lastname}
                valid={!errors.address_lastname}
                errorMessage={errors.address_lastname}
              />
              <TextInput
                type={'text'}
                value={values.street_name}
                onChange={handleChange}
                name={'street_name'}
                label={'Straße *'}
                placeholder={runnerData?.street_name}
                valid={!errors.street_name}
                errorMessage={errors.street_name}
              />
              <TextInput
                type={'text'}
                value={values.house_number}
                onChange={handleChange}
                name={'house_number'}
                label={'Hausnummer *'}
                placeholder={runnerData?.house_number}
                valid={!errors.house_number}
                errorMessage={errors.house_number}
              />
              <TextInput
                value={values.address_extra}
                onChange={handleChange}
                type={'text'}
                name={'address_extra'}
                label={'Adresszusatz'}
                placeholder={runnerData?.address_extra}
              />
              <TextInput
                type={'text'}
                value={values.postal_code}
                onChange={handleChange}
                name={'postal_code'}
                label={'PLZ *'}
                placeholder={runnerData?.postal_code}
                valid={!errors.postal_code}
                errorMessage={errors.postal_code}
              />
              <TextInput
                type={'text'}
                value={values.city}
                onChange={handleChange}
                name={'city'}
                label={'Stadt *'}
                placeholder={runnerData?.city}
                valid={!errors.city}
                errorMessage={errors.city}
              />
            </div>
            <h3>Zusätzliche Informationen:</h3>
            <TextInput
              type={'text'}
              value={values.start_number}
              onChange={handleChange}
              name={'start_number'}
              label={'Startnummer *'}
              placeholder={runnerData?.start_number}
            />
            <TextInput
              type={'text'}
              value={values.verification_code}
              onChange={handleChange}
              name={'verification_code'}
              label={'Verification Code (möglichst nicht ändern!)'}
              placeholder={runnerData?.verification_code}
            />
            <TextInput
              type={'text'}
              value={values.reason_for_payment}
              onChange={handleChange}
              name={'reason_for_payment'}
              label={'Verwendungszweck'}
              placeholder={runnerData?.reason_for_payment}
            />
            <span>
              Zahlungsstatus: {values.payment_status ? 'Bezahlt' : 'Zahlung ausstehend'} &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
              <Button
                name={'btn-confirm-payment'}
                label={values.payment_status ? 'Bezahlt' : 'Nicht bezahlt'}
                styling={values.payment_status ? 'paid-btn' : 'not-paid-btn'}
                type={'button'}
                onClick={() => {
                  change_payment_status(runner_id.toString(), !values.payment_status).then(() => {
                    values.payment_status = !values.payment_status;
                    setIsPageFound(false);
                  });
                }}
              />
            </span>
            <br />
            <br />
            Bestätigungsmail zur Zahlung:{' '}
            {runnerData?.payment_confirmation_mail_sent ? 'versendet' : 'noch nicht versendet'}
            <br />
            <br />
            <TextInput
              type={'text'}
              value={values.delivery_status}
              onChange={handleChange}
              name={'delivery_status'}
              label={'Lieferstatus'}
              placeholder={runnerData?.delivery_status}
            />
            <br />
            <Button
              name={'submitButton'}
              label={'Änderungen bestätigen'}
              type={'submit'}
              onSubmit={handleSubmit}
              styling={'brownbg'}
            />
          </div>
        </form>
      )}
    </div>
  );
};

export default Edit;
