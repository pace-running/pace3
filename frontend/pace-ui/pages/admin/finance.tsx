import { NextPage } from 'next';
import { useState } from 'react';
import { upload_payment_csv } from '../../apis/api';

const Finance: NextPage = () => {
  const [error, setError] = useState('');
  const [file, setFile] = useState<File>();


  const allowedExtensions = ['csv'];


  const handleFileChange = (e: any)  => {
    setError('');
    if (e.target.files.length) {
      const inputFile = e.target.files[0];
      const fileExtension = inputFile?.type.split('/')[1];
      if (!allowedExtensions.includes(fileExtension)) {
        setError('Please input a csv file');
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
       console.log(response);
      }
    }
  }

  // const cleanReason = (reason: string) => {
  //   return reason
  //     .trim()
  //     .split(/[\s,.]+/)
  //     .filter((text: string) => text.length === TOKEN_LENGTH)
  //     .map((text: string) => text.replace('0', 'O'))
  //     .map((text: string) => text.toUpperCase())
  //     .filter((text: string) => text.startsWith(TOKEN_PREFIX));
  // };



  return (
    <div>
      <h1>Finanzen</h1>
      <div>
        <label htmlFor='csvInput' style={{ display: 'block' }}>
          Hier .csv-Datei einfügen:
        </label>
        <input onChange={handleFileChange} id='csvInput' name='file' type='File' />
      </div>
      <br />
      <div>
        <button type='button' onClick={handleParse}>
          Einlesen
        </button>
      </div>
      {/* <div style={{ marginTop: '3rem' }}>
        {error ? error : data.map((col, idx) => <div key={idx}>{col.join(' ')}</div>)}
      </div> */}
      <div>
        {/* <table id='runnersTable' style={{ overflow: 'scroll' }}>
          <thead>
            <tr key={'head'}>
              <th>ID</th>
              <th>Verwendungszweck</th>
              <th>angegebene Spende</th>
              <th>erhaltener Betrag</th>
            </tr>
          </thead>
          <tbody>
            {wrongPayments.map((obj, key) => {
              return (
                <tr key={key}>
                  <td>{obj?.id}</td>
                  <td>{obj?.reason_for_payment}</td>
                  <td>{obj?.donation}</td>
                  <td>{obj?.received_amount}</td>
                </tr>
              );
            })}
          </tbody>
        </table> */}
      </div>
    </div>
  );
};

export default Finance;
