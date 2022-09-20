
import Link from "next/link";

const Footer: React.FC = () => {
  return (
    <footer id="footer" className="full footer brownbg p-3">
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
