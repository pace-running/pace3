import Image from 'next/image';
import router from 'next/router';
import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';
import React from 'react';
import { getThemeVar } from '../../utility/theme';

const Header: React.FC = () => {
  const eventName = getThemeVar('event_name');
  return (
    <Navbar className='navigation-bar' expand='md'>
      <Container>
        <Container>
          <Navbar.Brand href='/'>
            <span className='logo'>
              <Image src='/logo.svg' alt='FC St. Pauli Logo' height={100} width={100} />
              <span style={{ fontWeight: 'bold', fontSize: 24 }}> {eventName}</span>
            </span>
          </Navbar.Brand>
        </Container>
        <Nav className='m-auto'>
          <Nav.Link
            href='/join'
            id='header-button-registration'
            onClick={e => {
              e.preventDefault();
              router.push('/join');
            }}
          >
            <div className='navigation-button'>Hier Anmelden</div>
          </Nav.Link>
        </Nav>
      </Container>
    </Navbar>
  );
};

export default Header;
