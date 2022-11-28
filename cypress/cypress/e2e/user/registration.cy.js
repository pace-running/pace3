/// <reference types="Cypress" />


describe('should fill in registration form', () => {


    it('fills in basic information of runner', function () {


        cy.visit("https://pace3.lauf-gegen-rechts.de/")
        cy.get('[style="position:absolute;right:5%;border:1px solid white"] > .brownbg').click()

        cy.get(':nth-child(3) > .input-group > .form-control')
            .type('Vorname')
            .should('have.value', 'Vorname')
        cy.get(':nth-child(4) > .input-group > .form-control')
            .type('Nachname')
            .should('have.value', 'Nachname')
        cy.get(':nth-child(5) > .input-group > .form-control')
            .type('Team Name')
            .should('have.value', 'Team Name')
        cy.get(':nth-child(6) > .input-group > .form-control')
            .type('teamname@teamname.com')
            .should('have.value', 'teamname@teamname.com')
        cy.get(':nth-child(7) > .input-group > .form-control')
            .type('teamname@teamname.com')
            .should('have.value', 'teamname@teamname.com')
    })
    it('validates dropdown menus', function () {
        cy.get('#starting_point')
            .select(1).invoke("val")
            .should('eq', 'hamburg')
        cy.get('#starting_point')
            .select(0).invoke("val")
            .should('eq', 'Bitte auswählen')
        cy.get('#starting_point')
            .select(2).invoke("val")
            .should('eq', 'other')

        cy.get('#running_level')
            .select(0).invoke("val")
            .should('eq', 'Bitte auswählen')
        cy.get('#running_level')
            .select(1).invoke("val")
            .should('eq', 'rarely')
        cy.get('#running_level')
            .select(2).invoke("val")
            .should('eq', 'sometimes')
        cy.get('#running_level')
            .select(3).invoke("val")
            .should('eq', 'often')
    })
    it('fills in donation field', function () {
        cy.get(':nth-child(10) > .input-group > .form-control')
            .clear()
            .type('4')
            .should('have.value', '4')
        cy.get(':nth-child(10) > .invalid-feedback')
            .should('have.text', 'Die Spende muss mindestens 5€ betragen!')
        cy.get(':nth-child(10) > .input-group > .form-control')
            .clear()
            .type('5')
            .should('have.value', '5')
    })
    it('checks modal windows', function () {
        cy.get('[name="previewBtn"]').click()
        cy.get('.close-modal>span').click()
        cy.get('[name="sizesBtn"]').click()
        cy.get('.carousel-control-prev-icon').click()
        cy.get('.bi').click()
    })
    it('fills in t-shirt fields', function () {
    cy.get('#tshirt_toggle').click()
    cy.get('#tshirt_model')
        .select(0).invoke("val")
        .should('eq', 'Bitte auswählen')
    cy.get('#tshirt_model')
        .select(1).invoke("val")
        .should('eq', 'unisex')
    cy.get('#tshirt_size')
        .select(0).invoke("val")
        .should('eq', 'Bitte auswählen')
    cy.get('#tshirt_size')
        .select(1).invoke("val")
        .should('eq', 's')
    cy.get('#tshirt_size')
        .select(2).invoke("val")
        .should('eq', 'm')
    cy.get('#tshirt_size')
        .select(3).invoke("val")
        .should('eq', 'l')
    })
    //cy.get('#tshirt_model')
    //   .select(2).invoke("val")
    // .should('eq', 'slimfit')
    //cy.get('#tshirt_size')
    //  .select(0).invoke("val")
    //.should('eq', 'Bitte auswählen')
    //cy.get('#tshirt_size')
    //  .select(1).invoke("val")
    //.should('eq', 's')
})


