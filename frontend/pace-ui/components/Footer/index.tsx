import Link from 'next/link';
import React from 'react';
import { MDBFooter } from 'mdb-react-ui-kit';
import Container from 'react-bootstrap/Container';
import { getThemeVar } from '../../utility/theme';

const Footer: React.FC = () => {
  const eventName = getThemeVar('event_name');
  return (
    <MDBFooter className='text-center text-lg-start text-muted fixed-bottom brownbg'>
      <section className='d-flex p-4 border-bottom'>
        <Container style={{ width: '70%' }}>
          <span className='text-lg-center p-3 flex-column' style={{ color: '#FFFFFF', margin: 'auto', fontSize: 19 }}>
            {eventName}
          </span>
        </Container>
        <Container style={{ width: '70%' }}>
          <span className='text-center p-3 flex-column'>
            <Link href='/imprint'>
              <a className='styled-link'>Impressum</a>
            </Link>
          </span>
          <span className='text-center p-3 flex-column'>
            <Link href='/privacy_notice'>
              <a className='styled-link'>Datenschutz</a>
            </Link>
          </span>
        </Container>
      </section>
    </MDBFooter>
  );
};

export default Footer;
