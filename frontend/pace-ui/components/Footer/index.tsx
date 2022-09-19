import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import Link from "next/link";

const Footer = () => {
  return (
    <footer id="footer" className="full menu footer brownbg p-3 fixed-bottom">
      <div className="row">
        <Link href="/imprint">
          <a className="brownbg nav-link col-1">Impressum</a>
        </Link>
        <Link href="/privacy_notice">
          <a className="brownbg nav-link col-1">Datenschutz</a>
        </Link>
      </div>
    </footer>
  );
};

export default Footer;
