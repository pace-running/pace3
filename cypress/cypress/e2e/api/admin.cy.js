describe('admin api', () => {
  let base_url = Cypress.config('baseUrl') + 'api'
  it('rejects logins with wrong credentials', () => {
    let body = {
      username: "foo",
      password: "baz"
    }

    cy.request({
        method: 'POST',
        url: base_url + '/admin/login',
        failOnStatusCode: false,
        body: body,
    })
    .its('status').should('be.equal', 403)

  })
  it('rejects unauthenticated requests', () => {
    cy.request({
          url: base_url + '/admin/runners',
          failOnStatusCode: false,
      })
      .its('status').should('be.equal', 401)
  })
})

  
