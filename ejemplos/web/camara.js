let contenedor = document.createElement("div");
contenedor.style.display = "flex";
contenedor.style.flexDirection = "column";
contenedor.style.alignItems = "center";
contenedor.style.fontFamily = "sans-serif";
document.body.appendChild(contenedor);
let titulo = document.createElement("h1");
titulo.innerText = "📷 Cámara Web con ÁGUILA";
contenedor.appendChild(titulo);
let video = document.createElement("video");
video.autoplay = true;
video.playsInline = true;
video.style.width = "640px";
video.style.height = "480px";
video.style.backgroundColor = "#000";
video.style.borderRadius = "10px";
video.style.boxShadow = "0 4px 6px rgba(0,0,0,0.1)";
contenedor.appendChild(video);
let boton = document.createElement("button");
boton.innerText = "Iniciar Cámara";
boton.style.marginTop = "20px";
boton.style.padding = "10px 20px";
boton.style.fontSize = "18px";
boton.style.backgroundColor = "#28a745";
boton.style.color = "white";
boton.style.border = "none";
boton.style.borderRadius = "5px";
boton.style.cursor = "pointer";
contenedor.appendChild(boton);
function detenerTrack(track) {
track.stop();
}
function detenerCamara() {
let tracks = video.srcObject.getTracks();
tracks.forEach(detenerTrack);
video.srcObject = null /* Expresión no soportada */;
boton.innerText = "Iniciar Cámara";
boton.disabled = false;
boton.style.backgroundColor = "#28a745";
boton.onclick = iniciarCamara;
}
function errorCamara(error) {
alert("Error al acceder a la cámara: " + error);
boton.disabled = false;
boton.innerText = "Reintentar";
}
function camaraConectada(stream) {
video.srcObject = stream;
boton.innerText = "Cámara Activa";
boton.style.backgroundColor = "#dc3545";
boton.onclick = detenerCamara;
}
function iniciarCamara() {
boton.disabled = true;
boton.innerText = "Accediendo...";
let promesa = navigator.mediaDevices.getUserMedia({
"video": true,
"audio": false
});
promesa.then(camaraConectada);
promesa.catch(errorCamara);
}
boton.onclick = iniciarCamara;

module.exports = { contenedor, titulo, video, boton, detenerTrack, detenerCamara, errorCamara, camaraConectada, iniciarCamara };
