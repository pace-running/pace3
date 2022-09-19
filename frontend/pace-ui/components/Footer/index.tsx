import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";

const Footer = () => {
  return (
    <footer id="footer" className="full menu footer brownbg p-3 fixed-bottom">
      <div className="row">
        <a className="brownbg nav-link col-1" href="/imprint">
          Impressum
        </a>
        <a className="brownbg nav-link col-1" href="/privacy_notice">
          Datenschutz
        </a>
      </div>
    </footer>
  );
};

export default Footer;
