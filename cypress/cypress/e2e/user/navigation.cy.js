describe('should check if it is possible to navigate every page', () => {
    it("checks anmelden button", function () {
        cy.intercept("/api/theme").as("getTheme");
        cy.visit("/");
        cy.visit("/");
        cy.wait('@getTheme').then(_ => {
            cy.get('#header-button-registration')
                .should(($el) => {
                    expect($el).to.have.contain('Anmelden')
                })
                .click();
        });
    });
    it("checks Impressum link in the footer", function () {
        cy.visit("/")
        cy.get('[href="/imprint"]')
            .should(($el) => {
                expect($el).to.have.contain('Impressum')
            })
            .click()
            .url()
            .should('include', '/imprint')
        });

    
    

   
    
   

    })

