describe('should check if it is possible to navigate every page', () => {
    it("checks anmelden button", function () {
        cy.visit("/")
        cy.get('[style="padding-bottom:100px"] > :nth-child(2)')
            .should(($el) => {
                expect($el).to.have.contain('Anmelden')
            })
            .click()  

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

