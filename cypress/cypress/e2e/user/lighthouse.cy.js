context("Accessibility testing", () => {
  before(() => {
    cy.visit("/")
  });

  it("should verify the score of the main page", () => {
    cy.lighthouse({
      accessibility: 100,
      "best-practices": 100,
      seo: 50,
      pwa: 30,
      performance: 50,
    });

  });
  it("should verify the score of the join page", () => {
    cy.lighthouse({
      accessibility: 100,
      "best-practices": 100,
      seo: 50,
      pwa: 30,
      performance: 50,
    });
  });
  it("should verify the score of the imprint page", () => {
    cy.lighthouse({
      accessibility: 100,
      "best-practices": 100,
      seo: 50,
      pwa: 30,
      performance: 50,
    });
  });
  it("should verify the score of the admin login page", () => {
    cy.lighthouse({
      accessibility: 100,
      "best-practices": 100,
      seo: 50,
      pwa: 30,
      performance: 50,
    });
  });
});