import "../styles/globals.css";
import "bootstrap/dist/css/bootstrap.css";
import type { AppProps } from "next/app";
import JoinFormProvider from "../context/JoinFormContext";

function MyApp({ Component, pageProps: { ...pageProps } }: AppProps) {
  return (
    <JoinFormProvider>
      <Component {...pageProps} />
    </JoinFormProvider>
  );
}

export default MyApp;
