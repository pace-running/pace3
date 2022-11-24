import axios from 'axios';

export async function submitJoinInfo(data: InfoRequestData) {
  return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/runners`, data, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function fetchRunnerDetails(runner_id: string, verification_code: string) {
  return await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/runners/${runner_id}`, {
    params: { verification_code: verification_code },
    headers: { 'content-type': 'application/json' }
  });
}

export async function fetchAllRunners() {
  return await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/runners`, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function confirm_payment(runner_id: string) {
  return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/verification/${runner_id}`, runner_id, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function edit_runner(runner_id: string, data: fullRunnerData) {
  return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/full_runner/${runner_id}`, data, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function get_full_runner(runner_id: string) {
  return await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/full_runner/${runner_id}`, {
    headers: { 'content-type': 'application/json' }
  });
}