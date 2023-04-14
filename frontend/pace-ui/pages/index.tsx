import type { NextPage } from 'next';
import BaseLayout from '../components/Layout/baseLayout';
// import { useRouter } from 'next/router';
import Container from 'react-bootstrap/Container';
import Image from 'next/image';
import React from 'react';
import { getThemeVar } from '../utility/theme';

const Home: NextPage = () => {
  // const router = useRouter();
  const eventName = getThemeVar('event_name');
  const eventDescription = getThemeVar('event_description');
  return (
    <BaseLayout pageTitle={eventName}>
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
          <br />
          <div>
            <p> {eventDescription}</p>
          </div>
        </Container>
      </section>
    </BaseLayout>
  );
};

export default Home;
