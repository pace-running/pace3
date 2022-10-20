import axios from 'axios';

export async function submitJoinInfo(data: InfoRequestData) {
  return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/runners`, data, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function fetchRunnerDetails(runner_id: string) {
  return await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/runners/${runner_id}`, {
    headers: { 'content-type': 'application/json' }
  });
}
