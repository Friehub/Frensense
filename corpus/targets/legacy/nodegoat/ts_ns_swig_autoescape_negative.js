// SAFE: Enable autoescape in Swig to prevent XSS.

const swig = require("swig");
const express = require("express");
const consolidate = require("consolidate");
const app = express();

app.engine(".html", consolidate.swig);
app.set("view engine", "html");
app.set("views", `${__dirname}/app/views`);

swig.setDefaults({
    autoescape: true
});

http.createServer(app).listen(3000);