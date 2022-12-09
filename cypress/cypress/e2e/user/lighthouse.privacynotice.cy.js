context("Accessibility testing for privacy notice", () => {
    before(() => {
      cy.visit("http://pace3.lauf-gegen-rechts.de/privacy_notice");
    });
  
    it("should verify the score of the privacy notice page", () => {
      cy.lighthouse({
        accessibility: 95,
        "best-practices": 92,
        seo: 50,
        pwa: 30,
        performance: 95,
      });
  
    });
});