import type { NextPage } from 'next';
import BaseLayout from '../components/Layout/baseLayout';
// import { useRouter } from 'next/router';
import Container from 'react-bootstrap/Container';
import Image from 'next/image';
import React, { Component, useEffect, useState } from 'react';
import LoadingScreen from '../components/LoadingScreen';
import { getThemeVar, initTheme } from '../apis/api';

const Home: NextPage = () => {
  // const router = useRouter();
  const eventName = getThemeVar("event_name");
  return (
    <BaseLayout pageTitle={ eventName }>
      <section>
        <Container className='flex-row'>
          <div className='head-text'>
            <div className='head-image'>
              <Image src='/banner-alternative.jpg' height={608} width={800} />
            </div>
            <div className='vertical'></div>
            <div className='text-on-image'>
              <h3 style={{ fontSize: '50px' }}> placeholder image and text </h3>
            </div>
          </div>
        </Container>
        <Container>
          <div>
            <h3 style={{ fontSize: '50px', color: '#3a2b20' }}> HINTERGRUND </h3>
          </div>
          <div className='horizontal'></div>
          <div>
            <p>
              {' '}
              Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et
              dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex
              ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu
              fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt
              mollit anim id est laborum.
            </p>
          </div>
        </Container>
      </section>
    </BaseLayout>
  );
};

const Root: NextPage = () => {
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    initTheme().then(() => {
      console.log("setLoading false");
      setLoading(false);
    });
  }, []);

  if (loading) {
    console.log("Root: loading");
    return <LoadingScreen />
  } else {
    console.log("Root: finished loading");
    return <Home />
  }
};

export default Root;
