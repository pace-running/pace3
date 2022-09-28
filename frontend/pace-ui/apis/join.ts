import axios from 'axios';

export async function submitJoinInfo(data: InfoRequestData) {
    return await axios.post(`${process.env.NEXT_PUBLIC_API_URL}/api/submit`, data, {
        headers: {'content-type': 'application/json'}
    });
}