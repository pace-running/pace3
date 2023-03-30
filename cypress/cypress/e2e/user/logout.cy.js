describe("logging out", () => {
  it("logs out the user if logout button is clicked", () => {
    cy.visit("/admin/login");
    cy.get("#username_input").type("admin");
    cy.get("#password_input").type("xoh7Ongui4oo");
    cy.get('[data-testid="btn-login"]').click();

    cy.get('[data-testid="logout-btn"]').click();
    cy.contains("Login");
    cy.visit("/admin");
    cy.contains("Login");
  });
});
