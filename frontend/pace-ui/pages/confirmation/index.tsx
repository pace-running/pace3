import { NextPage } from 'next';
import BaseLayout from '../../components/Layout/baseLayout';
import React, { useEffect, useState } from 'react';
import RegistrationConfirmation from '../../running/RegistrationConfirmation';
import RunnerContext from '../../context/RunnerContext';

const ConfirmationPage: NextPage = () => {
  const { infoResponseData } = RunnerContext.useRunnerContext();
  const [responseData, setResponseData] = useState(infoResponseData);
  useEffect(() => {
    if (responseData.runner_id) {
      localStorage.setItem('responseData', JSON.stringify(responseData));
    }
  }, [responseData.runner_id]);

  useEffect(() => {
    const storedResponseData = JSON.parse(localStorage.getItem('responseData') ?? '{}');
    if (responseData) {
      setResponseData(storedResponseData);
    }
  }, []);

  return (
    <BaseLayout pageTitle='Anmeldungsbestätigung'>
      <RegistrationConfirmation responseData={responseData} />
    </BaseLayout>
  );
};

export default ConfirmationPage;
