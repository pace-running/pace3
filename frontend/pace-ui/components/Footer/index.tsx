import Link from "next/link";
import React from "react";

const Footer: React.FC = () => {
    return (
        <footer id="footer" className="full footer brownbg p-3 fixed-bottom">
            <div className="row">
                <Link href="/imprint">
                    <a className="brownbg nav-link col-1" style={{paddingLeft: "12px"}}>Impressum</a>
                </Link>
                <Link href="/privacy_notice">
                    <a className="brownbg nav-link col-1" style={{position: "absolute", left: "110px"}}>Datenschutz</a>
                </Link>
            </div>
        </footer>
    );
};

export default Footer;
