"use strict";
console.log("--- Pruebas de MATE ---");
console.log("PI: " + mate.pi);
console.log("Sin(0): " + mate.sin(0));
console.log("Cos(0): " + mate.cos(0));
console.log("Raiz(16): " + mate.raiz(16));
console.log("Potencia(2, 3): " + mate.potencia(2, 3));
console.log("Aleatorio: " + mate.aleatorio());
console.log("
--- Pruebas de FECHA ---");
let t = fecha.ahora();
console.log("Timestamp: " + t);
console.log("Fecha: " + fecha.formato(t, "%Y-%m-%d %H:%M:%S"));
console.log("
--- Pruebas de ASYNC/AWAIT ---");
async function tarea() {
console.log("Iniciando tarea...");
(await 1);
console.log("Tarea completada");
return "Hecho";
}
let res = (await tarea());
console.log("Resultado: " + res);
let f = async function() {
return "Anonima Async";
};
console.log((await f()));

module.exports = { t, tarea, res, f };
