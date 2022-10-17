describe('Lighthouse', function () {

    it('uses Lighthouse on main page', function () {
        cy.visit("https://pace3.lauf-gegen-rechts.de/")
        //cy.lighthouse()
        
        
    })
    it('uses Lighthouse on join page', function () {
        cy.visit("http://pace3.lauf-gegen-rechts.de/join")
        // cy.lighthouse()
        
        
    })
})

