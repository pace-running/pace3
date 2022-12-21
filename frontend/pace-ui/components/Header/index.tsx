import Image from 'next/image';
import Link from 'next/link';
import router from 'next/router';
import logo from '../../public/logo.svg';
import React from 'react';

const Header: React.FC = () => {
  return (
    <nav className='navbar p-3 full brownbg py-5' style={{ marginRight: '0' }}>
      <div className='text-left fixed-top' style={{ zIndex: '-1' }}>
        <Image src={logo} alt='FC St. Pauli Logo' height={100} width={100} />
      </div>
      <div className='text-left' style={{ position: 'absolute', left: '110px' }}>
        <Link href='/'>
          <a className='brownbg plain greyhover' style={{ textDecoration: 'none', fontSize: 'min(26px,4vw)' }}>
            Lauf gegen Rechts
          </a>
        </Link>
      </div>
      <div style={{ position: 'absolute', right: '5%', border: '1px solid white' }}>
        <button
        id='header-button-registration'
          className='brownbg'
          onClick={e => {
            e.preventDefault();
            router.push('/join');
          }}
        >
          Anmelden
        </button>
      </div>
    </nav>
  );
};

export default Header;
