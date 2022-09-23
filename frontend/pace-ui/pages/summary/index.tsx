import { NextPage } from "next";
import { useJoinFormContext } from "../../context/JoinFormContext";

const SummaryPage: NextPage = () => {
  const { joinFormData } = useJoinFormContext();

  return <div></div>;
};

export default SummaryPage;
