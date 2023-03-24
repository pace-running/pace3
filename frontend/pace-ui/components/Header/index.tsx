import Image from 'next/image';
import Link from 'next/link';
import router from 'next/router';
import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';
import React from 'react';

const Header: React.FC = () => {
  return (
    <Navbar className='navigation-bar' expand='lg'>
      <Container>
        <Navbar.Brand href='/'>
          <span className='logo'>
            <Image src='/logo.svg' alt='FC St. Pauli Logo' height={100} width={100} />
            <span style={{ fontWeight: 'bold', fontSize: 24 }}> Lauf gegen Rechts</span>
          </span>
        </Navbar.Brand>
        <Navbar.Toggle aria-controls='basic-navbar-nav' />
        <Nav className='me-auto' style={{ position: 'absolute', right: '5%' }}>
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
