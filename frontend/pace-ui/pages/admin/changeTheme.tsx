import type { NextPage } from 'next';
import { useFormik } from 'formik';
import { getThemeVar } from '../../utility/theme';
import { ChangeThemeFormValues, ChangeThemeSchema } from '../../utility/changeThemeSchema';
import TextInput from '../../components/TextInput';
import Checkbox from '../../components/Checkbox';
import Button from '../../components/Button';
import { updateTheme } from '../../apis/api';
import router from 'next/router';
import Head from 'next/head';
import { Helmet } from 'react-helmet';

const ChangeTheme: NextPage = () => {
  const submitForm = (values: ChangeThemeFormValues) => {
    updateTheme(values);
    router.push('/admin');
  };

  const { handleChange, setFieldValue, values, handleSubmit, errors, isValid } = useFormik<ChangeThemeFormValues>({
    initialValues: {
      eventTitle: getThemeVar('event_name'),
      eventDescription: getThemeVar('event_description'),
      closedRegistrationMessage: getThemeVar('closed_registration_message'),
      isRegistrationOpen: getThemeVar('is_registration_open') === 'true' ? true : false,
      tshirtsEnabled: getThemeVar('enable_tshirts') === 'true' ? true : false
    },
    validationSchema: ChangeThemeSchema,
    onSubmit: submitForm
  });

  return (
    <form onSubmit={handleSubmit}>
      <div style={{ margin: '50px' }}>
        <Helmet>
          <html lang='de' />
        </Helmet>
        <Head>
          <title>Adminbereich_/Seite_konfigurieren</title>
        </Head>
        <h1>Admin</h1>
        <h1>Seite konfigurieren</h1>
        <Button
          name={'back-btn-admin'}
          label={'Zurück zum Adminbereich'}
          type={'button'}
          onClick={() => {
            router.push('/admin');
          }}
        />
        <br />
        <br />

        <TextInput
          type={'text'}
          value={values.eventTitle}
          onChange={handleChange}
          name={'eventTitle'}
          label={'Titel des Events:'}
          valid={!errors.eventTitle}
          errorMessage={errors.eventTitle}
        />

        <TextInput
          type={'text'}
          value={values.eventDescription}
          onChange={handleChange}
          name={'eventDescription'}
          label={'Beschreibung des Events:'}
          valid={!errors.eventDescription}
          errorMessage={errors.eventDescription}
        />

        <TextInput
          type={'text'}
          value={values.closedRegistrationMessage}
          onChange={handleChange}
          name={'closedRegistrationMessage'}
          label={'Nachricht, falls Registrierung geschlossen ist:'}
          valid={!errors.closedRegistrationMessage}
          errorMessage={errors.closedRegistrationMessage}
        />

        <Checkbox
          name={'isRegistrationOpen'}
          check={values.isRegistrationOpen}
          label={'Ist die Registrierung geöffnet?'}
          role='switch'
          onChange={() => {
            setFieldValue('isRegistrationOpen', !values.isRegistrationOpen);
          }}
        />

        <Checkbox
          name={'tshirtsEnabled'}
          check={values.tshirtsEnabled}
          label={'Werden T-Shirts angeboten?'}
          role='switch'
          onChange={() => {
            setFieldValue('tshirtsEnabled', !values.tshirtsEnabled);
          }}
        />

        <Button
          name={'btn-submit-theme'}
          label={'Änderungen speichern!'}
          type={'submit'}
          onSubmit={handleSubmit}
          disabled={!isValid}
        />
      </div>
    </form>
  );
};

export default ChangeTheme;
