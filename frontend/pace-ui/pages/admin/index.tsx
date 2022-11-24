import type { NextPage } from 'next';
import router from 'next/router';
import React, { useEffect, useState } from 'react';
import { fetchAllRunners, change_payment_status } from '../../apis/api';
import Button from '../../components/Button';

const Admin: NextPage = () => {
  const [runnerList, setRunnerList] = useState<RunnerResponseData[]>();
  const [runnersLoaded, setRunnersLoaded] = useState(false);
  const [searchCategory, setSearchCategory] = useState('name');
  const [searchPrompt, setSearchPrompt] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSelectorContent, setPageSelectorContent] = useState(1);

  const rowsPerPage = 15;

  const filterRunnerList = function () {
    if (searchCategory === 'name') {
      setRunnerList(runnerList?.filter(runner => (runner.firstname + ' ' + runner.lastname).includes(searchPrompt)));
    } else if (searchCategory === 'start_number') {
      setRunnerList(runnerList?.filter(runner => runner.start_number == searchPrompt));
    } else if (searchCategory === 'email') {
      setRunnerList(runnerList?.filter(runner => runner.email.includes(searchPrompt)));
    } else if (searchCategory === 'reason_for_payment') {
      setRunnerList(runnerList?.filter(runner => runner.reason_for_payment.includes(searchPrompt)));
    }
  };

  useEffect(() => {
    const fetchRunners = async () => {
      if (!runnersLoaded) {
        console.log('Loading Runners');
        const response = await fetchAllRunners().catch(() => {});
        if (response?.status === 200) {
          // set contents with response data
          setRunnerList(response.data);
          setRunnersLoaded(true);
        } else {
          router.push('/admin/login');
        }
      }
    };
    filterRunnerList();
    fetchRunners();
  }, [runnersLoaded]);

  const radioChange = (e: { target: { value: React.SetStateAction<string>; }; }) => setSearchCategory(e.target.value);

  return (
    <div style={{ margin: '50px' }}>
      <h1>Admin</h1>
      <div>
        <h4>Statistiken:</h4>
        <p>Statistiken beziehen sich auf den angewendeten Filter!</p>
        <p>Läufer gesamt: {runnerList?.length}</p>
        <p>Läufer, die Hamburg starten: {runnerList&&runnerList.reduce<number>((acc: number,r: RunnerResponseData) => (r.starting_point==='hamburg'?acc+1:acc),0)}</p>
        <p>Spenden gesamt: {runnerList&&runnerList.reduce<number>((acc,r)=>acc+Number(r.donation),0)}</p>

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
            <p>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Startnummer &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
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
            <p>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Verwendungszweck &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</p>
          </label>
          <Button
          name={'btn-start-search'}
          label={'Start search'}
          type={'button'}
          onClick={() => {
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
            disabled={currentPage === 1}
            type={'button'}
            styling={'admin-btn'}
            onClick={() => {
              setCurrentPage(currentPage - 1);
            }}
          />
          &nbsp;&nbsp;&nbsp;
          {currentPage}/{runnerList? Math.max(1,Math.ceil(runnerList?.length/rowsPerPage)) : 1}
          &nbsp;&nbsp;&nbsp;
          <Button
            name={'btn-page-up'}
            label={'➡️'}
            disabled={currentPage * rowsPerPage >= (runnerList ? runnerList?.length : 0)}
            type={'button'}
            styling={'admin-btn'}
            onClick={() => {
              setCurrentPage(currentPage + 1);
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
              setCurrentPage(pageSelectorContent);
            }}
          />
        </span>
      </div>
      <br />

      <table id='runnersTable'>
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
          {runnerList
            ?.sort((a, b) => (a.id > b.id ? 1 : -1))
            ?.slice((currentPage - 1) * rowsPerPage, currentPage * rowsPerPage)
            ?.map((runner, key) => {
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
                      label={runner.payment_status?'Bezahlt':'Nicht bezahlt'}
                      styling={runner.payment_status?'paid-btn':'not-paid-btn'}
                      type={'button'}
                      onClick={() => {
                        change_payment_status(runner.id.toString());
                        setTimeout(()=>{setRunnersLoaded(false);},200);
                      }}
                    />
                  </td>
                  <td>
                    <Button
                      name={`btn-edit-runner-${runner.id}`}
                      label={'Bearbeiten'}
                      type={'button'}
                      onClick={() => {router.push({
                        pathname: '/admin/edit',
                        query: {id: runner.id.toString()}
                      });}}
                    />
                  </td>
                </tr>
              );
            })}
        </tbody>
      </table>
      <div style={{height: '100px'}}></div>
    </div>
  );
};

export default Admin;
