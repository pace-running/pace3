import type { NextPage } from 'next';
import router from 'next/router';
import React, { useEffect, useState } from 'react';
import { change_payment_status, fetchFilteredRunners } from '../../apis/api';
import Button from '../../components/Button';

const Admin: NextPage = () => {
  const [runnerList, setRunnerList] = useState<RunnerResponseData[]>();
  const [runnersLoaded, setRunnersLoaded] = useState(false);
  const [searchCategory, setSearchCategory] = useState('name');
  const [searchPrompt, setSearchPrompt] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSelectorContent, setPageSelectorContent] = useState(1);
  const [stats, setStats] = useState([0, 0, 0]);

  const rowsPerPage = 15;

  useEffect(() => {
    const fetchRunners = async () => {
      if (!runnersLoaded) {
        const response = await fetchFilteredRunners(currentPage, searchCategory, searchPrompt)//.catch(() => {});
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
    // filterRunnerList();
    fetchRunners();
  }, [runnersLoaded, currentPage]);

  const radioChange = (e: { target: { value: React.SetStateAction<string> } }) => setSearchCategory(e.target.value);

  return (
    <div style={{ margin: '50px' }}>
      <h1>Admin</h1>
      <Button
        name={'btn-finance'}
        label={'Zahlungsinformationen hochladen'}
        type={'button'}
        onClick={() => {
          router.push('/admin/finance');
        }}
      />
      <div>
        <h4>Statistiken:</h4>
        <p>Statistiken beziehen sich auf den angewendeten Filter!</p>
        <p>Läufer gesamt: {stats[0]}</p>
        <p>Läufer, die Hamburg starten: {stats[1]}</p>
        <p>Spenden gesamt: {stats[2]}</p>

        <h3>Suche:</h3>
        <div style={{ marginBottom: '20px' }}>
          <input
            type='text'
            name='search_prompt'
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
                &nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Startnummer &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
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
              <p>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; E-mail&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
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
              label={'Start search'}
              type={'button'}
              onClick={() => {
                setCurrentPage(1);
                setRunnersLoaded(false);
              }}
            />
          </span>
        </div>
      </div>

      <h2>Registrierte Läufer:</h2>

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
            <th>E-mail</th>
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
                      change_payment_status(runner.id.toString(), !runner.payment_status).then(() =>
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
      <div style={{ height: '100px' }}></div>
    </div>
  );
};

export default Admin;
