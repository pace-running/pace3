import Image from 'next/image';
import React from 'react';

const LoadingScreen: React.FC = () => {
  return (
    <div className='loading-screen'>
      <div>
        <Image src='/loading-orca.gif' alt='Lade-Icon' height={500} width={500} />
      </div>
      <p>Seite lädt...</p>
    </div>
  );
};

export default LoadingScreen;
