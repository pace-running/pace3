import type { NextPage } from 'next';
import React, { useEffect, useState } from 'react';
import { fetchAllRunners } from '../../apis/api';

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
      <>{console.log(runnerList)}</>
    </div>
  );
};

export default Admin;
