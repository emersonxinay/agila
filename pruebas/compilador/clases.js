class Animal {
    constructor(n) {
this.nombre = n;
    }
    hacer_sonido() {
console.log("...");
    }
}
class Perro extends Animal {
    hacer_sonido() {
console.log("Guau!");
    }
}
let p = new Perro("Firulais");
console.log("Nombre del perro:");
console.log(p.nombre);
console.log("Sonido:");
p.hacer_sonido();
let a = new Animal("Generico");
console.log("Nombre del animal:");
console.log(a.nombre);
console.log("Sonido:");
a.hacer_sonido();

module.exports = { Animal, Perro, p, a };
