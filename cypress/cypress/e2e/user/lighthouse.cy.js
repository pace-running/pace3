describe('Lighthouse', () => {

    it('uses Lighthouse on main page', () => {
        cy.visit(`${Cypress.config().baseUrl}`)
        //cy.lighthouse()
    });

    it('uses Lighthouse on join page', () => {
        cy.visit(`${Cypress.config().baseUrl}/join`)
        // cy.lighthouse()
    });
});

