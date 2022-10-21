context("Main page", () => {
    beforeEach(() => {
      cy.visit("https://pace3.lauf-gegen-rechts.de");
    });

  it("should verify the score of the main page", () => {
    cy.lighthouse({
      accessibility: 50,
      "best-practices": 50,
      seo: 50,
      pwa: 30,
      performance: 50,
    });
});
 
    });