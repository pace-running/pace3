describe('Form Anmelden', function () {

    it('Form Anmelden', function () {


        cy.visit("https://pace3.lauf-gegen-rechts.de/")
        cy.get('[style="position:absolute;right:5%;border:1px solid white"] > .brownbg').click()
        cy.get(':nth-child(3) > .input-group > .form-control').type('Vorname')
        cy.get(':nth-child(4) > .input-group > .form-control').type('Nachname')
        cy.get(':nth-child(5) > .input-group > .form-control').type('Team Name')
        cy.get(':nth-child(6) > .input-group > .form-control').type('teamname@teamname.com')
        cy.get(':nth-child(7) > .input-group > .form-control').type('teamname@teamname.coms')
        cy.get('#starting_point').select(1).invoke("val").should('eq', 'hamburg')
        cy.get('#starting_point').select(0).invoke("val").should('eq', 'Bitte auswählen')
        cy.get('#starting_point').select(2).invoke("val").should('eq', 'other')

    })
})
