import {NextPage} from "next";
import {useEffect, useState} from "react";
import BaseLayout from "../../components/Layout/baseLayout";
import {useJoinFormContext} from "../../context/JoinFormContext";

const SummaryPage: NextPage = () => {
    const {joinFormData} = useJoinFormContext();
    const [formData, setFormData] = useState(joinFormData);

    useEffect(() => {
        if (formData) {
            localStorage.setItem("formData", JSON.stringify(formData));
        }
    }, [formData]);

    useEffect(() => {
        const formData = JSON.parse(localStorage.getItem("formData") ?? "{}");
        if (formData) {
            setFormData(formData);
        }
    }, []);


    return (
        <BaseLayout pageTitle="Zusammenfassung">
            <div
                className="container"
                style={{maxWidth: "800px", textAlign: "center"}}
            >
                <h1>Zusammenfassung</h1>
                <p>Bitte überprüfe deine Daten</p>
                <div
                    style={{
                        textAlign: "left",
                        border: "3px solid grey",
                        margin: "30px",
                        padding: "20px",
                    }}
                >
                    <h2>PERSÖNLICHE ANGABEN</h2>

                    <p>
                        Name: {formData?.firstname} {formData?.lastname}
                    </p>
                    <p>Team: {formData?.team}</p>
                    <p>E-Mail: {formData?.email}</p>

                    {formData?.starting_point === "hamburg" ? (
                        <p>Startort: Hamburg</p>
                    ) : (
                        <p>Startort: Woanders</p>
                    )}

                    {formData?.running_level === "rarely" ? (
                        <p>Laufniveau: Ich laufe selten.</p>
                    ) : formData?.running_level === "often" ? (
                        <p>Laufniveau: Ich laufe gelegentlich bis regelmäßig.</p>
                    ) : (
                        <p>Laufniveau: Ich laufe häufig und ambitioniert.</p>
                    )}
                </div>

                {formData?.tshirt_toggle && (
                    <div
                        style={{
                            textAlign: "left",
                            display: "flex",
                            justifyContent: "center",
                            margin: "30px",
                            border: "3px solid grey",
                        }}
                    >
                        <div style={{textAlign: "left", padding: "20px"}}>
                            <h2>T-SHIRT ANGABEN</h2>
                            {formData?.tshirt_model === "unisex" ? (
                                <p>Modell: Unisex</p>
                            ) : (
                                <p>Modell: Tailliert</p>
                            )}

                            <p>
                                Größe:{" "}
                                <span style={{textTransform: "uppercase"}}>
                  {formData?.tshirt_size}
                </span>
                            </p>
                        </div>
                        <div style={{textAlign: "left", padding: "20px"}}>
                            <h2>LIEFERADRESSE</h2>
                            <p>
                                {formData?.address_firstname} {formData?.address_lastname}
                            </p>
                            <p>
                                {formData?.street_name} {formData?.house_number}
                            </p>
                            <p>
                                {formData?.postal_code} {formData?.city}
                            </p>
                            <p>{formData?.address_extra}</p>
                            <p>{formData?.country}</p>
                        </div>
                    </div>
                )}

                <div style={{textAlign: "left"}}>
                    <p>Spendenbeitrag: {formData?.donation}€</p>
                    <p>Versand: kostenlos (innerhalb Deutschland)</p>

                    <p>Zu zahlen: {formData?.donation}€</p>
                </div>
            </div>
        </BaseLayout>
    );
};

export default SummaryPage;
