describe("changing the password", () => {
  const changePassword = (oldPassword, newPassword) => {
    cy.get("#oldPassword_input").type(oldPassword);
    cy.get("#newPassword_input").type(newPassword);
    cy.get("#newPasswordRepeat_input").type(newPassword);
    cy.get('[data-testid="btn-confirm-new-password"]').click();
  };

  it("can change password and login with it", () => {
    cy.clearCookies();
    cy.visit("/admin/login");
    cy.get("#username_input").type("admin");
    cy.get("#password_input").type("xoh7Ongui4oo");
    cy.get('[data-testid="btn-login"]').click();

    cy.get('[data-testid="btn-change-password"]').click();
    changePassword("xoh7Ongui4oo", "newPassword");
    cy.location('pathname').should('eq', '/admin')

    cy.clearCookies();
    cy.visit("/admin/login");
    cy.get("#username_input").type("admin");
    cy.get("#password_input").type("newPassword");
    cy.get('[data-testid="btn-login"]').click();

    cy.contains("Registrierte Teilnehmende:");

    cy.visit("/change_password");
    changePassword("newPassword", "xoh7Ongui4oo");
  });

  it("shows an error when specifying the wrong old password", () => {
    cy.clearCookies();
    cy.visit("/admin/login");
    cy.get("#username_input").type("admin");
    cy.get("#password_input").type("xoh7Ongui4oo");
    cy.get('[data-testid="btn-login"]').click();

    cy.get('[data-testid="btn-change-password"]').click();
    // we are catching it in the original code, no idea why Cypress doesn't notice
    cy.once('uncaught:exception', () => false);
    changePassword("wrongpassword", "newPassword");

    // location did not change
    cy.location('pathname').should('eq', '/change_password')

    cy.contains("Das alte Passwort ist nicht korrekt");
  });
});
