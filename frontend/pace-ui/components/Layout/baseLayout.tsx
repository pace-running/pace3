import Header from '../../components/Header';
import Footer from '../../components/Footer';
import Head from 'next/head';
import React from 'react';
import { Helmet } from 'react-helmet';

type BaseLayoutProps = {
  children: React.ReactNode;
  pageTitle: string;
};

const BaseLayout: React.FC<BaseLayoutProps> = props => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', height: '100vh' }}>
      <Helmet>
        <html lang='de' />
      </Helmet>
      <Head>
        <title>{props.pageTitle}</title>
      </Head>

      <Header />
      <div style={{ flexGrow: '1', marginBottom: '2rem' }}>{props.children}</div>
      <Footer />
    </div>
  );
};

export default BaseLayout;
