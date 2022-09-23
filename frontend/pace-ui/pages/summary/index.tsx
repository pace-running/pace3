import { NextPage } from "next";
import BaseLayout from "../../components/Layout/baseLayout";
import { useJoinFormContext } from "../../context/JoinFormContext";

const SummaryPage: NextPage = () => {
  const { joinFormData } = useJoinFormContext();
  console.log(joinFormData);
  return (
    <BaseLayout pageTitle="Zusammenfassung">
      <div
        className="container"
        style={{ maxWidth: "800px", textAlign: "center" }}
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
            Name: {joinFormData?.firstname} {joinFormData?.lastname}
          </p>
          <p>Team: {joinFormData?.team}</p>
          <p>E-Mail: {joinFormData?.email}</p>

          {joinFormData?.starting_point === "hamburg" ? (
            <p>Startort: Hamburg</p>
          ) : (
            <p>Startort: Woanders</p>
          )}

          {joinFormData?.running_level === "rarely" ? (
            <p>Laufniveau: Ich laufe selten.</p>
          ) : joinFormData?.running_level === "often" ? (
            <p>Laufniveau: Ich laufe gelegentlich bis regelmäßig.</p>
          ) : (
            <p>Laufniveau: Ich laufe häufig und ambitioniert.</p>
          )}
        </div>

        {joinFormData?.tshirt_toggle && (
          <div
            style={{
              textAlign: "left",
              display: "flex",
              justifyContent: "center",
              margin: "30px",
              border: "3px solid grey",
            }}
          >
            <div style={{ textAlign: "left", padding: "20px" }}>
              <h2>T-SHIRT ANGABEN</h2>
              {joinFormData?.tshirt_model === "unisex" ? (
                <p>Modell: Unisex</p>
              ) : (
                <p>Modell: Tailliert</p>
              )}

              <p>
                Größe:{" "}
                <span style={{ textTransform: "uppercase" }}>
                  {joinFormData?.tshirt_size}
                </span>
              </p>
            </div>
            <div style={{ textAlign: "left", padding: "20px" }}>
              <h2>LIEFERADRESSE</h2>
              <p>
                {joinFormData?.address_firstname}{" "}
                {joinFormData?.address_lastname}
              </p>
              <p>
                {joinFormData?.street_name} {joinFormData?.house_number}
              </p>
              <p>
                {joinFormData?.postal_code} {joinFormData?.city}
              </p>
              <p>{joinFormData?.address_extra}</p>
              <p>{joinFormData?.country}</p>
            </div>
          </div>
        )}

        <div style={{ textAlign: "left" }}>
          <p>Spendenbeitrag: {joinFormData?.donation}€</p>
          <p>Versand: kostenlos (innerhalb Deutschland)</p>

          <p>Zu zahlen: {joinFormData?.donation}€</p>
        </div>
      </div>
    </BaseLayout>
  );
};

export default SummaryPage;
