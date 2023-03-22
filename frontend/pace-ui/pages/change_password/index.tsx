import { useFormik } from 'formik';
import type { NextPage } from 'next';
import TextInput from '../../components/TextInput';


const ChangePassword: NextPage = () => {
    return <div style={{ margin: '50px' }}>
        <h1>Admin</h1>
        <TextInput type='password' name='input-oldPassword' label='Altes Passwort' />
        <TextInput type='password' name='input-newPassword' label='Neues Passwort' />
        <TextInput type='password' name='input-newPasswordRepeat' label='Neues Passwort wiederholen' />
    </div>
};

export default ChangePassword;