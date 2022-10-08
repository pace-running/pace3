import React, {createContext, useContext, useState} from "react";

type RunnerContextType = {
    infoResponseData: InfoResponseData;
    setInfoResponseData: (data: InfoResponseData) => void;
};

const RunnerContext = createContext<RunnerContextType>({
    infoResponseData: {
        runner_id: "",
        start_number: "",
        donation: "",
        payment: "",
        email_provided: false,
    },
    setInfoResponseData: () => {
    }
});

const useRunnerContext = () => {
    const context = useContext(RunnerContext)
    if (!context) {
        throw new Error("Context error")
    }
    return context
}

const RunnerContextProvider: React.FC<any> = ({children}) => {
    const [runnerContextData, setRunnerContextData] = useState<InfoResponseData>({
        runner_id: "",
        start_number: "",
        donation: "",
        payment: "",
        email_provided: false,
    })

    return (
        <RunnerContext.Provider
            value={{
                infoResponseData: runnerContextData,
                setInfoResponseData: setRunnerContextData,
            }}
        >
            {children}
        </RunnerContext.Provider>
    );
};

export default {RunnerContext, RunnerContextProvider, useRunnerContext};