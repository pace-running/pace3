import "../styles/globals.css";
import "bootstrap/dist/css/bootstrap.css";
import type {AppProps} from "next/app";
import JoinFormProvider from "../context/JoinFormContext";
import RunnerContext from "../context/RunnerContext";

function MyApp({Component, pageProps: {...pageProps}}: AppProps) {
    return (
        <JoinFormProvider>
            <RunnerContext.RunnerContextProvider>
                <Component {...pageProps} />
            </RunnerContext.RunnerContextProvider>
        </JoinFormProvider>
    );
}

export default MyApp;
