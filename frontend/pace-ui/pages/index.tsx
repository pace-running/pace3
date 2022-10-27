import type { NextPage } from 'next';
import BaseLayout from '../components/Layout/baseLayout';
import { useRouter } from 'next/router';

const Home: NextPage = () => {
  const router = useRouter();
  return (
    <BaseLayout pageTitle='Lauf gegen Rechts'>
      <button
        className='brownbg'
        onClick={e => {
          e.preventDefault();
          router.push('/join');
        }}
      >
        Anmelden
      </button>
    </BaseLayout>
  );
};

export default Home;
