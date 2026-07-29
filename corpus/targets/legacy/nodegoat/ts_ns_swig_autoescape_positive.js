// [frensense]
// observation: Swig template engine has autoescape disabled globally, allowing unescaped user input in rendered HTML templates.
// impact: An attacker can inject arbitrary HTML and JavaScript into rendered pages, leading to cross-site scripting (XSS) attacks.
// improvement: Enable autoescape by setting it to true, or escape user input manually in templates.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021

const swig = require("swig");
const express = require("express");
const consolidate = require("consolidate");
const app = express();

app.engine(".html", consolidate.swig);
app.set("view engine", "html");
app.set("views", `${__dirname}/app/views`);

swig.setDefaults({
    autoescape: false
});

http.createServer(app).listen(3000);