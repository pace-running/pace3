import Image from 'next/image';
import React from 'react';

const LoadingScreen: React.FC = () => {
  return (
    <div className='loading-screen'>
      <div className='loader'></div>
      <p>Seite lädt</p>
    </div>
  );
};

export default LoadingScreen;
