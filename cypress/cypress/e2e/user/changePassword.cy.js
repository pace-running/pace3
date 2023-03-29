describe("changing the password", () => {
    
  after(() => {
    cy.visit("/change_password");
    changePassword("newPassword", "xoh7Ongui4oo");
  });

  const changePassword = (oldPassword, newPassword) => {
    cy.get("#oldPassword_input").type(oldPassword);
    cy.get("#newPassword_input").type(newPassword);
    cy.get("#newPasswordRepeat_input").type(newPassword);
    cy.get('[data-testid="btn-confirm-new-password"]').click();
  };

  it("logs in with old password", () => {
    cy.clearCookies();
    cy.visit("/admin/login");
    cy.get("#username_input").type("admin");
    cy.get("#password_input").type("xoh7Ongui4oo");
    cy.get('[data-testid="btn-login"]').click();

    cy.get('[data-testid="btn-change-password"]').click();
    changePassword("xoh7Ongui4oo", "newPassword");

    cy.clearCookies();
    cy.visit("/admin/login");
    cy.get("#username_input").type("admin");
    cy.get("#password_input").type("newPassword");
    cy.get('[data-testid="btn-login"]').click();

    cy.contains("Registrierte Teilnehmende:");
  });
});
