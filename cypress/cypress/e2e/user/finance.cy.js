describe("check rejected transactions", () => {
    beforeEach(() => {
        cy.clearCookies();
        cy.visit("/admin/login");
        cy.get("#username_input").type("admin");
        cy.get("#password_input").type("xoh7Ongui4oo");
        cy.get('[data-testid="btn-login"]').click();
      });

    it("shows rejected transactions after uploading a csv file", () => {
        cy.get('[name="btn-finance"]').click();
        cy.get('#csvInput').selectFile('test-files/Umsaetze.csv');
        cy.get('[data-testid="btn-upload"]').click();

        cy.contains("Upload erfolgreich, 0 Transaktionen bestätigt und 4 abgelehnt!")

        cy.get("tbody")
            .should("contain", "LGR-JAQPY")
            .and("contain", "LGR-YPKDM")
            .and("contain", "LGR-QVPMH")
            .and("contain", "LGR-TTZLK, LGR-WEGDS")
    });
});