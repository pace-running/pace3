import type { NextPage } from 'next';
import BaseLayout from '../../components/Layout/baseLayout';
import TextInput from '../../components/TextInput';
import Button from '../../components/Button';
import axios from 'axios';
import router from 'next/router';
import React from 'react';
import Head from 'next/head';
import { Helmet } from 'react-helmet';

const submitForm = async (event: React.SyntheticEvent) => {
  event.preventDefault();

  const formElements = (event.currentTarget as HTMLFormElement).elements as HTMLFormControlsCollection & {
    username: { value: string };
    password: { value: string };
  };
  const loginData = {
    username: formElements.username.value,
    password: formElements.password.value
  };
  await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/login`, loginData).then(response => {
    console.log(response);
    router.push('/admin');
  });
};

const Login: NextPage = () => {
  return (
    <BaseLayout pageTitle='Admin Login'>
      <form onSubmit={submitForm}>
        <div className='container'>
          <Helmet>
            <html lang='de' />
          </Helmet>
          <Head>
            <title>Adminbereich_/Login</title>
          </Head>
          <h1>Login</h1>
          <TextInput type={'text'} name={'username'} label={'Username'} />
          <TextInput type={'password'} name={'password'} label={'Passwort'} />
          <Button name={'submitButton'} label={'Login'} type={'submit'} styling={'brownbg'} testID={'btn-login'} />
        </div>
      </form>
    </BaseLayout>
  );
};

export default Login;
