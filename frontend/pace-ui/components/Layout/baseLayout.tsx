import Header from '../../components/Header';
import Footer from '../../components/Footer';
import Head from 'next/head';
import React from 'react';

type BaseLayoutProps = {
  children: React.ReactNode;
  pageTitle: string;
};

const BaseLayout: React.FC<BaseLayoutProps> = props => {
  return (
    <div style={{ paddingBottom: '100px' }}>
      <Head>
        <title>{props.pageTitle}</title>
      </Head>

      <Header />
      {props.children}
      <Footer />
    </div>
  );
};

export default BaseLayout;
