const {defineConfig} = require("cypress");
const {lighthouse, prepareAudit} = require("@cypress-audit/lighthouse");


module.exports = {
    e2e: {
        baseUrl: "http://localhost:8089/", // this is your app
        setupNodeEvents(on, config) {
            on("before:browser:launch", (browser = {}, launchOptions) => {
                prepareAudit(launchOptions);
            });

            on("task", {
                lighthouse: lighthouse(),
            });
        },
    },
};

