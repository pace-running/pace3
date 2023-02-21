import Link from 'next/link';
import React from 'react';
import { MDBFooter} from 'mdb-react-ui-kit';

const Footer: React.FC = () => {
  return (
    <MDBFooter className='text-center text-lg-start text-muted fixed-bottom brownbg' >
    <div className='text-center p-4' style={{ backgroundColor: 'rgba(0, 0, 0, 0.05)' }}>
      <Link className='text-reset fw-bold' href='/imprint'>
        <a style={{ color: 'white'}}> 
          Impressum
        </a>
      </Link>
    </div>
    
    <div className='text-center p-4'> 
      <Link className='text-reset fw-bold' href='/privacy_notice'>
       <a style={{ color: 'white'}}>
          Datenschutz
       </a>
      </Link>
    </div>
</MDBFooter>
  );
};

export default Footer;
