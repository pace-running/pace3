import { NextPage } from 'next';
import { useState } from 'react';
import { upload_payment_csv } from '../../apis/api';

const Finance: NextPage = () => {
  const [error, setError] = useState('');
  const [file, setFile] = useState<File>();
  const [wrongPayments, setWrongPayments] = useState<FaultyTransaction[]>();

  const allowedExtensions = ['csv'];

  const handleFileChange = (e: any) => {
    setError('');
    if (e.target.files.length) {
      const inputFile = e.target.files[0];
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
      const response = await upload_payment_csv(file);
      if (response?.status === 200) {
        setWrongPayments(response.data);
      }
    } else {
      setError('Bitte wähle zunächst eine Datei aus!');
    }
  };

  return (
    <div style={{ margin: '50px' }}>
      <h1>Finanzen</h1>
      <div>
        <label htmlFor='csvInput' style={{ display: 'block' }}>
          Hier .csv-Datei einfügen:
        </label>
        <input onChange={handleFileChange} id='csvInput' name='file' type='File' />
      </div>
      <br />
      <p>{error}</p>
      <div>
        <button type='button' onClick={handleParse}>
          Einlesen
        </button>
      </div>
      <br />
      <br />
      <div>
        <table id='runnersTable' style={{ overflow: 'scroll' }}>
          <thead>
            <tr key={'head'}>
              <th>ID</th>
              <th>Verwendungszweck</th>
              <th>erhaltener Betrag</th>
              <th>erwarteter Betrag</th>
            </tr>
          </thead>
          <tbody>
            {wrongPayments &&
              wrongPayments.map((obj, key) => {
                return (
                  <tr key={key}>
                    <td>{obj?.runner_ids ? obj?.runner_ids : 'Teilnehmer nicht gefunden'}</td>
                    <td>{obj?.reason_for_payment}</td>
                    <td>{obj?.amount}</td>
                    <td>{obj?.expected_amount}</td>
                  </tr>
                );
              })}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default Finance;
