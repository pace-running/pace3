import type { NextPage } from 'next';
import router from 'next/router';
import React, { useEffect, useState } from 'react';
import { fetchAllRunners, verify_payment as confirm_payment } from '../../apis/api';
import Button from '../../components/Button';

const Admin: NextPage = () => {
  const [runnerList, setRunnerList] = useState<RunnerResponseData[]>();
  const [runnersLoaded, setRunnersLoaded] = useState(false);

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

    fetchRunners();
  }, [runnersLoaded]);

  return (
    <div>
      <h1>Admin</h1>
      <h2>Registered Runners:</h2>
      <table id='runnersTable'>
        <thead>
          <tr key={'head'}>
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
            ?.map((runner, key) => {
              return (
                <tr key={key}>
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
                        setRunnersLoaded(false);
                      }}
                    />
                  </td>
                  <td>
                    <Button name={`btn-edit-runner-${runner.id}`}
                    label={'Edit Runner'}
                    type={'button'}
                    onClick={()=>{}}
                    />
                  </td>
                </tr>
              );
            })}
        </tbody>
      </table>
    </div>
  );
};

export default Admin;
