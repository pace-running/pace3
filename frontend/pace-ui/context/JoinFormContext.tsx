import React, { createContext, useContext, useState } from "react";
import { JoinFormValues } from "../pages/join/joinFormSchema";

type JoinFormContextType = {
  joinFormData: JoinFormValues;
  setJoinFormData: (data: JoinFormValues) => void;
};

const JoinFormContext = createContext<JoinFormContextType>({});

const JoinFormProvider: React.FC = ({ children }) => {
  const [joinFormContextData, setJoinFormContextData] =
    useState<JoinFormValues>();

  return (
    <JoinFormContext.Provider
      value={{
        joinFormData: joinFormContextData,
        setJoinFormData: setJoinFormContextData,
      }}
    >
      {children}
    </JoinFormContext.Provider>
  );
};

export const useJoinFormContext = () => useContext(JoinFormContext);

export default JoinFormProvider;
