import type { NextPage } from 'next';
import BaseLayout from '../components/Layout/baseLayout';
import { useRouter } from 'next/router';

const Home: NextPage = () => {
  const router = useRouter();
  return (
    <BaseLayout pageTitle='Lauf gegen Rechts'>
    </BaseLayout>
  );
};

export default Home;
