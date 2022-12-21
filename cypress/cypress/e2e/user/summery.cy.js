describe('should visit summary page', () => {
    before(() => {
        cy.visit("http://localhost:8089/join");
        cy.get(':nth-child(10) > .input-group > .form-control')
            .clear()
            .type('5')
        cy.get('#starting_point')
            .select(1).invoke("val")
        cy.get('#running_level')
            .select(1).invoke("val")
        cy.get('#tos_confirmed')
            .check()
        cy.get('.container > .brownbg')
            .click()
    });
    it("checks the field start point", function () {
        cy.get('[style="text-align: left; border: 3px solid grey; margin: 30px; padding: 20px;"] > :nth-child(2)')
            .should(($el) => {
                expect($el).to.have.contain('Hamburg')
            })

    })
    it("checks the field running level", function () {
        cy.get('[style="text-align: left; border: 3px solid grey; margin: 30px; padding: 20px;"] > :nth-child(3)')
            .should(($el) => {
                expect($el).to.have.contain('selten')
            })

    })
    it("checks the donation field", function () {
        cy.get('[style="text-align: left; border: 3px solid grey; margin: 30px; padding: 20px;"] > :nth-child(3)')
            .should(($el) => {
                expect($el).to.have.contain('selten')
            })

    })
    it("submits the registration data", function () {


        cy.get('[type="submit"]').click()


    })
})