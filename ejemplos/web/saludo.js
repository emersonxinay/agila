console.log("Iniciando script ÁGUILA en el navegador...");
let titulo = document.createElement("h1");
titulo.innerText = "¡Hola desde ÁGUILA!";
titulo.style.color = "blue";
titulo.style.fontFamily = "sans-serif";
document.body.appendChild(titulo);
let parrafo = document.createElement("p");
parrafo.innerText = "Este HTML fue generado dinámicamente por código ÁGUILA compilado a JavaScript.";
document.body.appendChild(parrafo);
function saludar() {
alert("¡Has hecho clic en el botón!");
}
let boton = document.createElement("button");
boton.innerText = "Haz clic aquí";
boton.onclick = saludar;
boton.style.padding = "10px 20px";
boton.style.fontSize = "16px";
boton.style.cursor = "pointer";
document.body.appendChild(boton);

module.exports = { titulo, parrafo, saludar, boton };
