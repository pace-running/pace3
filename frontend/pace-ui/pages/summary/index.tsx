import { NextPage } from 'next';
import { useCallback, useEffect, useState } from 'react';
import BaseLayout from '../../components/Layout/baseLayout';
import { useJoinFormContext } from '../../context/JoinFormContext';
import Button from '../../components/Button';
import { useRouter } from 'next/router';
import { JoinFormValues } from '../../utility/joinFormSchema';
import { submitJoinInfo } from '../../apis/api';
import RunnerContext from '../../context/RunnerContext';

const mapJoinFormDataToRequestData = (formData: JoinFormValues) => ({
  firstname: formData.firstname ?? '',
  lastname: formData.lastname ?? '',
  team: formData.team ?? '',
  email: formData.email ?? '',
  repeat: formData.repeated_email ?? '',
  starting_point: formData.starting_point ?? '',
  running_level: formData.running_level ?? '',
  donation: formData.donation.toString(),
  confirm: formData.tos_confirmed ? 'on' : '',
  tshirt_toggle: formData.tshirt_toggle ? 'on' : '',
  tshirt_model: formData.tshirt_model ?? '',
  tshirt_size: formData.tshirt_size ?? '',
  country: formData.country ?? '',
  address_firstname: formData.address_firstname ?? '',
  address_lastname: formData.address_lastname ?? '',
  street_name: formData.street_name ?? '',
  house_number: formData.house_number ?? '',
  address_extra: formData.address_extra ?? '',
  postal_code: formData.postal_code ?? '',
  city: formData.city ?? ''
});

const SummaryPage: NextPage = () => {
  const router = useRouter();
  const { joinFormData } = useJoinFormContext();
  const [formData, setFormData] = useState(joinFormData);

  const { setInfoResponseData } = RunnerContext.useRunnerContext();

  useEffect(() => {
    if (formData) {
      localStorage.setItem('formData', JSON.stringify(formData));
    }
  }, [formData]);

  useEffect(() => {
    const formData = JSON.parse(localStorage.getItem('formData') ?? '{}');
    if (formData) {
      setFormData(formData);
    }
    // console.log(`Tshirt cost in formData: ${formData.tshirt_cost}`);
  }, []);

  const handleSubmit = useCallback(async () => {
    if (formData) {
      const response = await submitJoinInfo(mapJoinFormDataToRequestData(formData));
      if (response.data.status_code === 200) {
        const runner_id = response.data.runner_id.toString();
        const start_number = response.data.start_number.toString();
        const donation = response.data.donation.toString();
        const tshirt_cost = response.data.tshirt_cost.toString();
        const payment = response.data.reason_for_payment.toString();
        const verification_code = response.data.verification_code.toString();
        const email_provided = response.data.email_provided as boolean;
        setInfoResponseData({
          runner_id,
          start_number,
          donation,
          payment,
          email_provided,
          verification_code,
          tshirt_cost
        });
        await router.push({
          pathname: '/confirmation',
          query: { runner_id, start_number, donation, tshirt_cost, payment, email_provided }
        });
      }
    }
  }, [formData]);

  return (
    <BaseLayout pageTitle='Zusammenfassung'>
      <div className='container' style={{ maxWidth: '800px', textAlign: 'center' }}>
        <h1>Zusammenfassung</h1>
        <p>Bitte überprüfe deine Daten</p>
        <div
          style={{
            textAlign: 'left',
            border: '3px solid grey',
            margin: '30px',
            padding: '20px'
          }}
        >
          <h2>PERSÖNLICHE ANGABEN</h2>

          {(formData?.firstname || formData?.lastname) && (
            <p>
              Name: {formData?.firstname} {formData?.lastname}
            </p>
          )}
          {formData?.team && <p>Team: {formData?.team}</p>}
          {formData?.email && <p>E-Mail: {formData?.email}</p>}

          {formData?.starting_point === 'hamburg' ? <p>Startort: Hamburg</p> : <p>Startort: Woanders</p>}

          {formData?.running_level === 'rarely' ? (
            <p>Laufniveau: Ich laufe selten.</p>
          ) : formData?.running_level === 'often' ? (
            <p>Laufniveau: Ich laufe gelegentlich bis regelmäßig.</p>
          ) : (
            <p>Laufniveau: Ich laufe häufig und ambitioniert.</p>
          )}
        </div>

        {formData?.tshirt_toggle && (
          <div
            style={{
              textAlign: 'center',
              display: 'flex',
              margin: '30px',
              border: '3px solid grey'
            }}
          >
            <div style={{ textAlign: 'left', padding: '20px' }}>
              <h2>T-SHIRT ANGABEN</h2>
              {formData?.tshirt_model === 'unisex' ? <p>Modell: Unisex</p> : <p>Modell: Tailliert</p>}

              <p>
                Größe:
                <span style={{ textTransform: 'uppercase' }}> {formData?.tshirt_size}</span>
              </p>
            </div>
            <div style={{ textAlign: 'left', padding: '20px' }}>
              <h2>LIEFERADRESSE</h2>
              <p>
                {formData?.address_firstname} {formData?.address_lastname}
              </p>
              <p>
                {formData?.street_name} {formData?.house_number}
              </p>
              <p>
                {formData?.postal_code} {formData?.city}
              </p>
              <p>{formData?.address_extra}</p>
              <p>{formData?.country}</p>
            </div>
          </div>
        )}

        <div style={{ textAlign: 'left', margin: '30px', padding: '20px' }}>
          <p>Spendenbeitrag: {formData?.donation}€</p>
          {formData?.tshirt_toggle && (
            <div>
              <p>T-Shirt-Kosten: 15 €</p>
              <p>
                Versand:{' '}
                {formData?.tshirt_cost === 15
                  ? 'kostenlos (innerhalb Deutschland)'
                  : (formData.tshirt_cost - 15).toString() + '€'}
              </p>
            </div>
          )}
          <hr></hr>
          <p style={{ fontWeight: 'bold' }}>
            Zu zahlen:{' '}
            {formData?.donation && formData?.tshirt_cost
              ? formData?.donation + formData?.tshirt_cost
              : formData?.donation}
            €
          </p>
        </div>
        <div style={{ textAlign: 'center' }}>
          <Button
            name={'goBackButton'}
            label={'Zurück zur Bearbeitung'}
            type={'button'}
            onClick={() => {
              router.back();
            }}
            styling={'brownbg'}
          />
          {'   '}
          <Button name={'submitButton'} label={'Anmelden'} type={'submit'} onClick={handleSubmit} styling={'brownbg'} />
        </div>
      </div>
    </BaseLayout>
  );
};

export default SummaryPage;
