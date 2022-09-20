import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import logo from "../../public/logo.svg";
import styles from "./Header.module.css";

const Header: NextPage = () => {
  return (
    <nav className="navbar navbar-expand-lg p-3 full brownbg row">
      <div className="col text-left">
        <a className='brownbg' href="/">
          Lauf gegen Rechts
        </a>
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
