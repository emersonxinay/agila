function sumar(a, b) {
return a + b;
}
let res = sumar(10, 20);
console.log("Suma: ");
console.log(res);
function scope_test() {
let res = 500;
console.log("Dentro de funcion: ");
console.log(res);
}
scope_test();
console.log("Fuera de funcion (debe ser 30): ");
console.log(res);
