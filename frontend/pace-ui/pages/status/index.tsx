import type { NextPage } from 'next';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { fetchRunnerDetails } from '../../apis/api';
import BaseLayout from '../../components/Layout/baseLayout';
import StatusContent from '../../running/StatusContent';

const StatusPage: NextPage = () => {
  const router = useRouter();
  const runner_id = router.query.runner_id as string;
  const verification_code = router.query.verification_code as string;

  const [statusContent, setStatusContent] = useState<StatusResponseData>();
  const [isPageFound, setIsPageFound] = useState(false);

  useEffect(() => {
    const fetchData = async () => {
      if (runner_id && verification_code) {
        const response = await fetchRunnerDetails(runner_id, verification_code);
        if (response.status === 200) {
          // set contents with response data
          setStatusContent(response.data);
          setIsPageFound(true);
        }
      }
    };

    fetchData();
  }, [runner_id, verification_code]);

  return (
    <BaseLayout pageTitle='Status'>
      {isPageFound ? <StatusContent statusContent={statusContent} /> : <h1>Page Not Found</h1>}
    </BaseLayout>
  );
};

export default StatusPage;
