import { NextPage } from 'next';
import router from 'next/router';
import { useState } from 'react';
import { uploadPaymentCSV } from '../../apis/api';
import Button from '../../components/Button';

const Finance: NextPage = () => {
  const [error, setError] = useState('');
  const [file, setFile] = useState<File>();
  const [rejectedPayments, setRejectedPayments] = useState<RejectedTransaction[]>();
  const [uploadFeedback, setUploadFeedback] = useState<number[]>();

  const allowedExtensions = ['csv'];

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setError('');
    if (e.currentTarget.files?.length) {
      const inputFile = e.currentTarget.files[0];
      const fileExtension = inputFile?.type.split('/')[1];
      if (!allowedExtensions.includes(fileExtension)) {
        setError('Die Datei muss im .csv-Format sein!');
        return;
      }

      // If input type is correct set the state
      setFile(inputFile);
    }
  };

  const handleParse = async () => {
    if (file) {
      console.log('Uploading csv file...');
      const uploadResponse = await uploadPaymentCSV(file);
      if (uploadResponse?.status === 200) {
        setUploadFeedback(uploadResponse.data);
      } else {
        setUploadFeedback([-1, -1]);
        console.log('Error uploading the csv file!');
      }
    } else {
      setError('Bitte wähle zunächst eine Datei aus!');
    }
  };

  return (
    <div style={{ margin: '50px' }}>
      <h1>Finanzen</h1>
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
      <h2>Zahlungsinformationen Einlesen</h2>
      <div>
        <label htmlFor='csvInput' style={{ display: 'block' }}>
          Hier .csv-Datei einfügen:
        </label>
        <input onChange={handleFileChange} id='csvInput' name='file' type='File' />
      </div>
      <br />
      <p style={{ color: 'red' }}>{error}</p>
      <div>
        <button type='button' onClick={handleParse}>
          Einlesen
        </button>
      </div>
      <br />
      {uploadFeedback &&
        (uploadFeedback[0] === -1 ? (
          <div>Beim Upload ist etwas schiefgelaufen!</div>
        ) : (
          <div>
            Upload erfolgreich, {uploadFeedback[0]} Transaktionen bestätigt und {uploadFeedback[1]} abgelehnt!
          </div>
        ))}
      <br />
      <h3>Zu überprüfende Transaktionen</h3>
      <div>...</div>
    </div>
  );
};

export default Finance;
