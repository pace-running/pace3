import { useFormik } from 'formik';
import type { NextPage } from 'next';
import TextInput from '../../components/TextInput';
import Button from '../../components/Button';
import router from 'next/router';
import * as Yup from 'yup';
import { savePassword } from '../../apis/api';
import { useState } from 'react';
import { AxiosResponse } from 'axios';

type ChangePasswordValues = {
  oldPassword?: string;
  newPassword?: string;
  newPasswordRepeat?: string;
};

const ChangePasswordSchema = Yup.object().shape({
  oldPassword: Yup.string().required('Bitte geben Sie das alte Passwort ein'),
  newPassword: Yup.string()
    .required('Bitte geben Sie das neue Passwort ein')
    .notOneOf([Yup.ref('oldPassword'), null], 'Passwort darf nicht identisch mit dem alten Passwort sein'),
  newPasswordRepeat: Yup.string()
    .required('Bitte geben Sie das neue Passwort wiederholt ein')
    .oneOf([Yup.ref('newPassword'), null], 'Passwörter stimmen nicht überein')
});

const ChangePassword: NextPage = () => {
  const [serverError, setServerError] = useState('');
  const submitForm = async (values: ChangePasswordValues) => {
    console.log('submitting change password form...');
    setServerError('');

    try {
      await savePassword({ oldPassword: values.oldPassword, newPassword: values.newPassword });
      await router.push('/admin');
    } catch (response: any) {
      if (response as AxiosResponse) {
        setServerError((response as AxiosResponse).data.errorMessage);
      } else {
        console.error('Unknown error: ', response);
      }
    }
  };
  const { handleChange, values, handleSubmit, errors, isValid } = useFormik<ChangePasswordValues>({
    initialValues: {
      oldPassword: '',
      newPassword: '',
      newPasswordRepeat: ''
    },
    validationSchema: ChangePasswordSchema,
    onSubmit: submitForm
  });
  return (
    <div style={{ margin: '50px' }}>
      <h1>Admin</h1>
      <Button
        name={'back-btn-admin'}
        label={'Zurück zum Adminbereich'}
        type={'button'}
        onClick={() => {
          router.push('/admin');
        }}
      />
      <br />
      <form onSubmit={handleSubmit}>
        <div style={{ maxWidth: '300px' }}>
          <h2>Passwort ändern</h2>
          <TextInput
            type='password'
            name='oldPassword'
            label='Altes Passwort'
            value={values.oldPassword}
            valid={!errors.oldPassword}
            errorMessage={errors.oldPassword}
            onChange={handleChange}
          />
          <TextInput
            type='password'
            name='newPassword'
            label='Neues Passwort'
            value={values.newPassword}
            valid={!errors.newPassword}
            errorMessage={errors.newPassword}
            onChange={handleChange}
          />
          <TextInput
            type='password'
            name='newPasswordRepeat'
            label='Neues Passwort wiederholen'
            value={values.newPasswordRepeat}
            valid={!errors.newPasswordRepeat}
            errorMessage={errors.newPasswordRepeat}
            onChange={handleChange}
          />

          <div>{serverError}</div>

          <Button
            name='btn-savePassword'
            label='Passwort speichern'
            type='submit'
            onSubmit={handleSubmit}
            disabled={!(values.oldPassword && isValid)}
          />
        </div>
      </form>
    </div>
  );
};

export default ChangePassword;
