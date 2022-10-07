import {NextPage} from "next";
import BaseLayout from "../../components/Layout/baseLayout";
import React from "react";
import RegistrationConfirmation from "../../running/RegistrationConfirmation";
import {useRouter} from "next/router";

const ConfirmationPage: NextPage = () => {
    const router = useRouter();
    const donation = router.query.donation as string;
    const payment = router.query.payment as string;
    const emailProvided = (router.query.emailProvided as string) == 'true';
    return (
        <BaseLayout pageTitle="Anmeldungsbestätigung">
            <RegistrationConfirmation donation={donation} emailProvided={emailProvided} payment={payment}/>
        </BaseLayout>
    );
}

export default ConfirmationPage;