import '../styles/globals.css';
import 'bootstrap/dist/css/bootstrap.css';
import type { AppProps } from 'next/app';
import JoinFormProvider from '../context/JoinFormContext';
import RunnerContext from '../context/RunnerContext';
import { useEffect, useState } from 'react';
import { initTheme } from '../utility/theme';
import LoadingScreen from '../components/LoadingScreen';

function MyApp({ Component, pageProps: { ...pageProps } }: AppProps) {
  const [isLoading, setLoading] = useState(true);

  let ignore = false;
  useEffect(() => {
    if (!ignore) {
      ignore = true;
      initTheme().then(() => {
        setLoading(false);
      });
    }
  }, []);

  if (isLoading) return <LoadingScreen />;
  return (
    <JoinFormProvider>
      <RunnerContext.RunnerContextProvider>
        <Component {...pageProps} />
      </RunnerContext.RunnerContextProvider>
    </JoinFormProvider>
  );
}

export default MyApp;
