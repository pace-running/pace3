import { NextPage } from 'next';
import router from 'next/router';
import { useEffect, useState } from 'react';
import { getAllRejectedTransactions, uploadPaymentCSV, logOutUser, deleteFaultyTransactions } from '../../apis/api';
import Button from '../../components/Button';
import Checkbox from '../../components/Checkbox';
import Modal from '../../components/Modal';

const Finance: NextPage = () => {
  const [error, setError] = useState('');
  const [file, setFile] = useState<File>();
  const [rejectedPayments, setRejectedPayments] = useState<RejectedTransaction[]>();
  const [uploadFeedback, setUploadFeedback] = useState<number[]>();
  const [transactionsLoaded, setTransactionsLoaded] = useState(false);
  const [checkboxStates, setCheckboxStates] = useState(new Set<number>());
  const [showDeletionModal, setShowDeletionModal] = useState(false);

  const allowedExtensions = ['csv'];

  useEffect(() => {
    const fetchTransactions = async () => {
      if (!transactionsLoaded) {
        const response = await getAllRejectedTransactions();
        if (response?.status === 200) {
          setRejectedPayments(response.data);
          setTransactionsLoaded(true);
        }
      }
    };
    fetchTransactions();
  }, [transactionsLoaded]);

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
      setTransactionsLoaded(false);
    } else {
      setError('Bitte wähle zunächst eine Datei aus!');
    }
  };

  return (
    <div style={{ margin: '3rem', paddingBottom: '5rem' }}>
      <h1>Finanzen</h1>
      <Button
        name={'back-btn-admin'}
        label={'Zurück zum Adminbereich'}
        type={'button'}
        onClick={() => {
          router.push('/admin');
        }}
      />
      &nbsp;&nbsp;&nbsp;
      <Button
        name={'logout-btn'}
        label={'Ausloggen'}
        type={'button'}
        onClick={() => {
          logOutUser();
          router.push('/admin/login');
        }}
        testID='logout-btn'
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
        <Button name={'btn-upload'} label={'Einlesen'} type={'button'} onClick={handleParse} testID='btn-upload' />
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
      <div>
        <table id='rejectedPaymentsTable' style={{ overflow: 'scroll' }}>
          <thead>
            <tr key={'head'}>
              <th>Auswahl zum Löschen</th>
              <th>Datum</th>
              <th>Teilnehmenden IDs</th>
              <th>Verwendungszweck</th>
              <th>Betrag</th>
              <th>Erwarteter Betrag</th>
              <th>Währung</th>
              <th>Name</th>
              <th>IBAN</th>
            </tr>
          </thead>
          <tbody>
            {rejectedPayments &&
              rejectedPayments.map((transaction, key) => {
                return (
                  <tr key={key}>
                    <td>
                      <Checkbox
                        name={`checkbox-${transaction.id}`}
                        label='Löschen'
                        testID={`checkbox-${transaction.id}`}
                        check={checkboxStates.has(transaction.id)}
                        onChange={() => {
                          const newState = new Set(checkboxStates);
                          if (checkboxStates.has(transaction.id)) {
                            newState.delete(transaction.id);
                          } else {
                            newState.add(transaction.id);
                          }
                          setCheckboxStates(newState);
                        }}
                      />
                    </td>
                    <td>{transaction.date_of_payment}</td>
                    <td>{transaction.runner_ids}</td>
                    <td>{transaction.reasons_for_payment}</td>
                    <td>{transaction.payment_amount}</td>
                    <td>{transaction.expected_amount}</td>
                    <td>{transaction.currency}</td>
                    <td>{transaction.payer_name}</td>
                    <td>{transaction.iban}</td>
                  </tr>
                );
              })}
          </tbody>
        </table>
        <br />
        <Button
          name={'btn-open-deletion-modal'}
          label={'Ausgewählte Transaktionen löschen'}
          type={'button'}
          onClick={() => {
            setShowDeletionModal(true);
          }}
        />
        <Modal name={'confirmDeletionModal'} onClose={() => setShowDeletionModal(false)} open={showDeletionModal}>
          <h4>Sind Sie sicher, dass Sie die ausgewählten Transaktionen löschen möchten?</h4>
          <div>
            <Button
              name={'btn-confirm-deletion'}
              label={'Ja, löschen'}
              type={'button'}
              onClick={() => {
                deleteFaultyTransactions(Array.from(checkboxStates));
                setTransactionsLoaded(false);
                setCheckboxStates(new Set<number>());
                setShowDeletionModal(false);
              }}
            />
            <Button
              name={'btn-cancel-deletion'}
              label={'Zurück'}
              type={'button'}
              onClick={() => {
                setShowDeletionModal(false);
              }}
            />
          </div>
        </Modal>
      </div>
    </div>
  );
};

export default Finance;
