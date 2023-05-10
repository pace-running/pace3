import type { NextPage } from 'next';
import router from 'next/router';
import React, { useEffect, useState } from 'react';
import { changePaymentStatus, fetchFilteredRunners, logOutUser } from '../../apis/api';
import Button from '../../components/Button';
import LoadingScreen from '../../components/LoadingScreen';
import Checkbox from '../../components/Checkbox';
import Head from 'next/head';
import { Helmet } from 'react-helmet';

const Admin: NextPage = () => {
  const [runnerList, setRunnerList] = useState<RunnerResponseData[]>();
  const [runnersLoaded, setRunnersLoaded] = useState(false);
  const [searchCategory, setSearchCategory] = useState('name');
  const [searchPrompt, setSearchPrompt] = useState('');
  const [showOnlyBsv, setShowOnlyBsv] = useState(false);
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSelectorContent, setPageSelectorContent] = useState(1);
  const [stats, setStats] = useState([0, 0, 0]);

  const rowsPerPage = 15;

  useEffect(() => {
    const fetchRunners = async () => {
      if (!runnersLoaded) {
        const response = await fetchFilteredRunners(currentPage, searchCategory, searchPrompt, showOnlyBsv).catch(
          () => {}
        );
        if (response?.status === 200) {
          // set contents with response data
          setRunnerList(response.data.runner_list);
          setStats([response.data.stats_number, response.data.stats_hamburg, response.data.stats_total_donation]);
          setRunnersLoaded(true);
        } else {
          router.push('/admin/login');
        }
      }
    };
    fetchRunners();
  }, [runnersLoaded, currentPage]);

  const radioChange = (e: { target: { value: React.SetStateAction<string> } }) => setSearchCategory(e.target.value);
  if (runnersLoaded) {
    return (
      <div style={{ margin: '50px' }}>
        <Helmet>
          <html lang='de' />
        </Helmet>
        <Head>
          <title>Adminbereich</title>
        </Head>
        <h1>Admin</h1>
        <Button
          name={'btn-finance'}
          label={'Zahlungsinformationen hochladen'}
          type={'button'}
          onClick={() => {
            router.push('/admin/finance');
          }}
        />
        &nbsp;&nbsp;&nbsp;
        <Button
          name={'btn-change-theme'}
          label={'Seite konfigurieren'}
          type={'button'}
          onClick={() => {
            router.push('/admin/changeTheme');
          }}
        />
        &nbsp;&nbsp;&nbsp;
        <Button
          name={'btn-change_password'}
          label={'Passwort ändern'}
          type={'button'}
          onClick={() => {
            router.push('/change_password');
          }}
          testID={'btn-change-password'}
        />
        &nbsp;&nbsp;&nbsp;
        <Button
          name={'btn-logout'}
          label={'Ausloggen'}
          type={'button'}
          onClick={() => {
            logOutUser();
            router.push('/admin/login');
          }}
          testID='logout-btn'
        />
        <div>
          <h4>Statistiken:</h4>
          <p>Statistiken beziehen sich auf den angewendeten Filter!</p>
          <p data-testid='total-runners-p'>Teilnehmende gesamt: {stats[0]}</p>
          <p className='starting-hamburg'>Teilnehmende, die Hamburg starten: {stats[1]}</p>
          <p className='total-donation'>Spenden gesamt: {stats[2]}</p>

          <h3>Suche:</h3>
          <div style={{ marginBottom: '20px' }}>
            <input
              type='text'
              name='search_prompt'
              aria-label='Suchbegriff'
              value={searchPrompt}
              style={{ width: '50%' }}
              onChange={e => setSearchPrompt(e.target.value)}
            />
            <br />
          </div>
          <div>
            <span>
              <label>
                <input
                  type='radio'
                  value='start_number'
                  name='search_condition'
                  className='form-check-input'
                  onChange={radioChange}
                />{' '}
                <p>
                  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Startnummer
                  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                </p>
              </label>

              <label>
                <input
                  type='radio'
                  value='name'
                  name='search_condition'
                  className='form-check-input'
                  onChange={radioChange}
                />
                <p>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Name&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
              </label>

              <label>
                <input
                  type='radio'
                  value='email'
                  name='search_condition'
                  className='form-check-input'
                  onChange={radioChange}
                />
                <p>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; E-Mail&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
              </label>

              <label>
                <input
                  type='radio'
                  value='reason_for_payment'
                  name='search_condition'
                  className='form-check-input'
                  onChange={radioChange}
                />
                <p>
                  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Verwendungszweck
                  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
                </p>
              </label>
              <Button
                name={'btn-start-search'}
                label={'Suche starten'}
                type={'button'}
                onClick={() => {
                  setCurrentPage(1);
                  setRunnersLoaded(false);
                }}
              />
            </span>
          </div>
          <br />
          <Checkbox
            name={'checkbox-show-bsv'}
            label={'Nur BSV-Teilnehmer anzeigen (nach Team sortiert)'}
            check={showOnlyBsv}
            onChange={() => {
              setShowOnlyBsv(!showOnlyBsv);
              setRunnersLoaded(false);
            }}
          />
        </div>
        <h2>Registrierte Teilnehmende:</h2>
        <div>
          <span>
            <Button
              name={'btn-page-down'}
              label={'⬅'}
              disabled={currentPage <= 1}
              type={'button'}
              styling={'admin-btn'}
              onClick={() => {
                setCurrentPage(currentPage - 1);
                setRunnersLoaded(false);
              }}
            />
            &nbsp;&nbsp;&nbsp;
            {currentPage}/{runnerList ? Math.max(1, Math.ceil(stats[0] / rowsPerPage)) : 1}
            &nbsp;&nbsp;&nbsp;
            <Button
              name={'btn-page-up'}
              label={'➡️'}
              disabled={currentPage * rowsPerPage >= stats[0]}
              type={'button'}
              styling={'admin-btn'}
              onClick={() => {
                setCurrentPage(currentPage + 1);
                setRunnersLoaded(false);
              }}
            />
            &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
            <input
              type={'text'}
              value={pageSelectorContent}
              style={{ width: '5%' }}
              placeholder={'Seitenzahl'}
              aria-label={'Seitenzahl'}
              onChange={e => {
                setPageSelectorContent(+e.target.value);
              }}
            />
            &nbsp;&nbsp;&nbsp;
            <Button
              name={'btn-go-to-page'}
              type={'button'}
              label={'Gehe zu Seite'}
              onClick={() => {
                const maxPage = Math.ceil(stats[0] / rowsPerPage);
                let targetPage = pageSelectorContent;
                if (Number.isNaN(pageSelectorContent) || pageSelectorContent < 1) targetPage = 1;
                if (pageSelectorContent > maxPage) targetPage = maxPage;
                setPageSelectorContent(targetPage);
                setCurrentPage(targetPage);
                setRunnersLoaded(false);
              }}
            />
          </span>
        </div>
        <br />
        <table id='runnersTable' style={{ overflow: 'scroll' }}>
          <thead>
            <tr key={'head'}>
              <th>ID</th>
              <th>Startnummer</th>
              <th>Name</th>
              <th>Team</th>
              <th>E-Mail</th>
              <th>Spende</th>
              <th>Verwendungszweck</th>
              <th>🤑✅</th>
              <th>✍️</th>
            </tr>
          </thead>
          <tbody>
            {runnerList?.map((runner, key) => {
              return (
                <tr key={key}>
                  <td>{runner.id}</td>
                  <td>{runner.start_number}</td>
                  <td>
                    {runner.firstname} {runner.lastname}
                  </td>
                  <td>{runner.team}</td>
                  <td>{runner.email}</td>
                  <td>{runner.donation}</td>
                  <td>{runner.reason_for_payment}</td>
                  <td>
                    <Button
                      name={`btn-confirm-payment-${runner.id}`}
                      label={runner.payment_status ? 'Bezahlt' : 'Nicht bezahlt'}
                      styling={runner.payment_status ? 'paid-btn' : 'not-paid-btn'}
                      type={'button'}
                      onClick={() => {
                        console.log(`Changed status of runner ${runner.id}`);
                        changePaymentStatus(runner.id.toString(), !runner.payment_status).then(() =>
                          setRunnersLoaded(false)
                        );
                      }}
                    />
                  </td>
                  <td>
                    <Button
                      name={`btn-edit-runner-${runner.id}`}
                      label={'Bearbeiten'}
                      type={'button'}
                      onClick={() => {
                        router.push({
                          pathname: '/admin/edit',
                          query: { id: runner.id.toString() }
                        });
                      }}
                    />
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
        <div style={{ height: '10rem' }}></div>
      </div>
    );
  } else {
    return <LoadingScreen />;
  }
};

export default Admin;
