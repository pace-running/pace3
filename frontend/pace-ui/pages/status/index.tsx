import type { NextPage } from 'next';
import { useRouter } from 'next/router';
import { useCallback, useState } from 'react';
import { fetchRunnerDetails } from '../../apis/api';
import BaseLayout from '../../components/Layout/baseLayout';
import StatusContent from '../../running/StatusContent';

const StatusPage: NextPage = () => {
  const router = useRouter();
  const runner_id = router.query.runner_id as string;

  const [statusContent, setStatusContent] = useState<StatusResponseData>();
  const [isPageFound, setIsPageFound] = useState(false);
  let counter = 1;

  const loadContent = useCallback(async () => {
    if (runner_id && counter > 0) {
      const response = await fetchRunnerDetails(runner_id);
      if (response.data.status_code === 200) {
        // set contents with response data
        setStatusContent(response.data);
        setIsPageFound(true);
      }
      counter--;
    }
  }, [runner_id]);
  loadContent(); // This gets called repeatedly, workaround with counter should be temporary

  return (
    <BaseLayout pageTitle='Status'>
      {isPageFound ? <StatusContent statusContent={statusContent} /> : <h1>Page Not Found</h1>}
    </BaseLayout>
  );
};

export default StatusPage;
