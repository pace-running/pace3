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

export async function fetchFilteredRunners(page_number: number, search_category: string, search_keyword: string) {
  return await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/runners`, {
    params: { page_number,  search_category, search_keyword },
    headers: { 'content-type': 'application/json' }
  });
}

export async function change_payment_status(runner_id: string, truth: boolean) {
  return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/payment/${runner_id}`, truth, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function edit_runner(runner_id: string, data: FullRunnerData) {
  console.log(`PUT request with data of ${data.firstname} ${data.lastname}`);
  return await axios.put(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/full_runner/${runner_id}`, data, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function get_full_runner(runner_id: string) {
  return await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/full_runner/${runner_id}`, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function upload_payment_csv(file: File) {
  return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/finance`, file, {
    headers: { 'content-type': 'multipart/form-data' }
  });
}
