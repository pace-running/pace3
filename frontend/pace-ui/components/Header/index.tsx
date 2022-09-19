import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import logo from "../../public/logo.svg";
import styles from "./Header.module.css";

const Header: NextPage = () => {
  return (
    <nav className="navbar navbar-expand-lg p-3 brownbg row">
      <span>
        <a className={styles.fcsp} href="/">
          Lauf gegen Rechts
        </a>
      </span>
      <div className="float-end col-2">
        <Image
          src={logo}
          alt="FC St. Pauli Logo"
          height={80}
          width={80}
          layout={"fill"}
        />
      </div>
    </nav>
  );
};

export default Header;
