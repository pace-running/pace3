import type { NextPage } from 'next';
import { useRouter } from 'next/router';
import { useCallback, useMemo, useState } from 'react';
import { fetchRunnerDetails } from '../../apis/api';
import BaseLayout from '../../components/Layout/baseLayout';
import StatusContent from '../../running/StatusContent';

const StatusPage: NextPage = () => {
  const router = useRouter();
  const runner_id = router.query.runner_id as string;
  const verification_code = router.query.verification_code as string;

  const [statusContent, setStatusContent] = useState<StatusResponseData>();
  const [isPageFound, setIsPageFound] = useState(false);

  useMemo(async () => {
    if (!isPageFound && runner_id && verification_code) {
      const response = await fetchRunnerDetails(runner_id, verification_code);
      if (response.data.status_code === 200) {
        // set contents with response data
        setStatusContent(response.data);
        setIsPageFound(true);
      }
    }
  }, [runner_id, verification_code]);

  return (
    <BaseLayout pageTitle='Status'>
      {isPageFound ? <StatusContent statusContent={statusContent} /> : <h1>Page Not Found</h1>}
    </BaseLayout>
  );
};

export default StatusPage;
