import { useFormik } from 'formik';
import type { NextPage } from 'next';
import TextInput from '../../components/TextInput';
import Button from '../../components/Button';
import router from 'next/router';

const ChangePassword: NextPage = () => {
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
      <br/>
      <h2>Passwort ändern</h2>
      <TextInput type='password' name='input-oldPassword' label='Altes Passwort' />
      <TextInput type='password' name='input-newPassword' label='Neues Passwort' />
      <TextInput type='password' name='input-newPasswordRepeat' label='Neues Passwort wiederholen' />
      <Button
        name={'btn-savePassword'}
        label={'Passwort speichern'}
        type={'button'}
        disabled={true}
        onClick={() => {
          //router.push('/');
        }}
      />
    </div>
  );
};

export default ChangePassword;
