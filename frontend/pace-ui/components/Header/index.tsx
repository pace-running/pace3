import Image from "next/image";
import Link from "next/link";
import logo from "../../public/logo.svg";


const Header: React.FC = () => {
  return (
    <nav className="navbar navbar-expand-lg p-3 full brownbg row">
      <div className="col text-left">
        <Link href="/">
        <a className='brownbg' >
          Lauf gegen Rechts
        </a></Link>
      </div>
      <div className="col text-right">
        <Image
          src={logo}
          alt="FC St. Pauli Logo"
          height={80}
          width={80}
        />
      </div>
    </nav>
  );
};

export default Header;
