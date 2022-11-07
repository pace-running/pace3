import type { NextPage } from 'next';
import React, { useEffect, useState } from 'react';
import { fetchAllRunners, verify_payment } from '../../apis/api';
import Button from '../../components/Button';

const Admin: NextPage = () => {
  const [runnerList, setRunnerList] = useState<RunnerResponseData[]>();
  const [runnersLoaded, setRunnersLoaded] = useState(false);

  useEffect(() => {
    const fetchRunners = async () => {
      if (!runnersLoaded) {
        console.log('Loading Runners');
        const response = await fetchAllRunners();
        if (response.status === 200) {
          // set contents with response data
          setRunnerList(response.data);
          setRunnersLoaded(true);
        }
      }
    };

    fetchRunners();
  }, []);

  return (
    <div>
      <h1>Admin</h1>
      {/* <>{console.log(`Runners List: ${runnerList}`)}</> */}
      <h2>Registered Runners:</h2>
      <div>
        <table id='runnersTable'>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Team</th>
            <th>Starting point</th>
            <th>Running Level</th>
            <th>Donation</th>
            <th>Payment Status</th>
          </tr>
          {runnerList?.map((runner,key)=>{
            return (
              <tr key={key}>
                <td>{runner.id}</td>
                <td>{runner.firstname} {runner.lastname}</td>
                <td>{runner.team}</td>
                <td>{runner.starting_point}</td>
                <td>{runner.running_level}</td>
                <td>{runner.donation}</td>
                <td>{runner.payment_status?'True':'False'}</td>
                <td>
                  <Button name={`btn-verify-payment-${runner.id}`} label={'Verify Payment'} type={'button'} onClick={()=>{
                    verify_payment(runner.id.toString());
                  }}/>
                </td>
              </tr>
            );
          })}
        </table>
      </div>
    </div>
  );
};

export default Admin;
