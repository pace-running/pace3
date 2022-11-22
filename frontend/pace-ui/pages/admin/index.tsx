import type { NextPage } from 'next';
import router from 'next/router';
import React, { useEffect, useState } from 'react';
import { fetchAllRunners, verify_payment as confirm_payment } from '../../apis/api';
import Button from '../../components/Button';

const Admin: NextPage = () => {
  const [runnerList, setRunnerList] = useState<RunnerResponseData[]>();
  const [runnersLoaded, setRunnersLoaded] = useState(false);
  const [searchCategory, setSearchCategory] = useState('name');
  const [searchPrompt, setSearchPrompt] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSelectorContent, setPageSelectorContent] = useState(1);

  const rowsPerPage = 10;

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
        <h3>Search:</h3>
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
          <label>
            <input
              type='radio'
              value='start_number'
              name='search_condition'
              className='form-check-input'
              onChange={radioChange}
            />{' '}
            <p>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Start number</p>
          </label>
          <br />
          <label>
            <input
              type='radio'
              value='name'
              name='search_condition'
              className='form-check-input'
              onChange={radioChange}
            />
            <p>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Name</p>
          </label>
          <br />
          <label>
            <input
              type='radio'
              value='email'
              name='search_condition'
              className='form-check-input'
              onChange={radioChange}
            />
            <p>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Email</p>
          </label>
          <br />
          <label>
            <input
              type='radio'
              value='reason_for_payment'
              name='search_condition'
              className='form-check-input'
              onChange={radioChange}
            />
            <p>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Reason for payment</p>
          </label>
          <br />
        </div>
        <Button
          name={'btn-start-search'}
          label={'Start search'}
          type={'button'}
          onClick={() => {
            setRunnersLoaded(false);
          }}
        />
      </div>
      <h2>Registered Runners:</h2>

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
            placeholder={'enter page number'}
            onChange={e => {
              setPageSelectorContent(+e.target.value);
            }}
          />
          &nbsp;&nbsp;&nbsp;
          <Button
            name={'btn-go-to-page'}
            type={'button'}
            label={'go to page'}
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
            <th>Start number</th>
            <th>Name</th>
            <th>Team</th>
            <th>Email</th>
            <th>Donation</th>
            <th>Reason for payment</th>
            <th></th>
            <th></th>
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
                      label={'Confirm Payment'}
                      type={'button'}
                      disabled={runner.payment_status}
                      onClick={() => {
                        confirm_payment(runner.id.toString());
                        setTimeout(()=>{setRunnersLoaded(false);},200);
                      }}
                    />
                  </td>
                  <td>
                    <Button
                      name={`btn-edit-runner-${runner.id}`}
                      label={'Edit Runner'}
                      type={'button'}
                      onClick={() => {router.push(`admin/edit/${runner.id}`);}}
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
