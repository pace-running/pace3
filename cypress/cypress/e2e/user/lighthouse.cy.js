context("Main page", () => {
  before(() => {
    cy.visit("https://pace3.lauf-gegen-rechts.de");
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
    cy.visit("https://pace3.lauf-gegen-rechts.de/join");
    cy.lighthouse({
      accessibility: 100,
      "best-practices": 100,
      seo: 50,
      pwa: 30,
      performance: 50,
    });
  });
  it("should verify the score of the imprint page", () => {
    cy.visit("https://pace3.lauf-gegen-rechts.de/imprint");
    cy.lighthouse({
      accessibility: 100,
      "best-practices": 100,
      seo: 50,
      pwa: 30,
      performance: 50,
    });
  });
  it("should verify the score of the admin login page", () => {
    cy.visit("https://pace3.lauf-gegen-rechts.de/admin/login");
    cy.lighthouse({
      accessibility: 100,
      "best-practices": 100,
      seo: 50,
      pwa: 30,
      performance: 50,
    });
  });
});