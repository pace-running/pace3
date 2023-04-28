import axios, { AxiosPromise } from 'axios';
import { ThemeVars } from '../utility/theme';
import { ChangeThemeFormValues } from '../utility/changeThemeSchema';

export async function fetchTheme(): Promise<AxiosPromise<ThemeVars>> {
  return axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/theme`, {});
}

export async function updateTheme(data: ChangeThemeFormValues) {
  return await axios.put(`${process.env.NEXT_PUBLIC_API_URL}/api/theme`, data, {
    headers: { 'content-type': 'application/json' }
  });
}

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

export async function fetchFilteredRunners(
  page_number: number,
  search_category: string,
  search_keyword: string,
  show_only_bsv = false
) {
  return await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/runners`, {
    params: { page_number, search_category, search_keyword, show_only_bsv },
    headers: { 'content-type': 'application/json' }
  });
}

export async function changePaymentStatus(runner_id: string, truth: boolean) {
  return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/payment/${runner_id}`, truth, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function editRunner(runner_id: string, data: FullRunnerData) {
  console.log(`PUT request with data of ${data.firstname} ${data.lastname}`);
  return await axios.put(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/full_runner/${runner_id}`, data, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function getFullRunner(runner_id: string) {
  return await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/full_runner/${runner_id}`, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function uploadPaymentCSV(file: File) {
  return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/finance`, file, {
    headers: { 'content-type': 'multipart/form-data' }
  });
}

export async function getAllRejectedTransactions() {
  return await axios.get(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/finance`, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function savePassword(data: { oldPassword?: string; newPassword?: string }): Promise<AxiosPromise> {
  console.log('PUT request with new password');
  return axios.put(
    `${process.env.NEXT_PUBLIC_API_URL}/api/admin/change_password`,
    {
      // make it Rust-y
      old_password: data.oldPassword,
      new_password: data.newPassword
    },
    {
      headers: { 'content-type': 'application/json' }
    }
  );
}
export async function logOutUser() {
  return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/logout`, {
    headers: { 'content-type': 'application/json' }
  });
}

export async function deleteFaultyTransactions(ids: number[]) {
  return await axios.put(`${process.env.NEXT_PUBLIC_API_URL}/api/admin/finance/delete`, ids, {
    headers: { 'content-type': 'application/json' }
  });
}
