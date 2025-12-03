#!/bin/bash

# Script para probar todos los ejemplos de Águila
# Colores para output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0
TOTAL=0

echo "========================================="
echo "  Probando Ejemplos de Águila"
echo "========================================="
echo ""

# Función para ejecutar un test
# Función para ejecutar un test
run_test() {
    local file=$1
    local name=$2
    local input=$3
    TOTAL=$((TOTAL + 1))
    
    echo -n "[$TOTAL] Probando $name... "
    
    if [ -n "$input" ]; then
        # Use printf to handle newlines correctly
        if printf "$input" | ./aguila/target/release/aguila ejecutar "$file" > /dev/null 2>&1; then
            echo -e "${GREEN}✓ PASS${NC}"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}✗ FAIL${NC}"
            FAILED=$((FAILED + 1))
            echo "    Archivo: $file"
        fi
    else
        if ./aguila/target/release/aguila ejecutar "$file" > /dev/null 2>&1; then
            echo -e "${GREEN}✓ PASS${NC}"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}✗ FAIL${NC}"
            FAILED=$((FAILED + 1))
            echo "    Archivo: $file"
        fi
    fi
}

# BÁSICOS
echo -e "${YELLOW}=== BÁSICOS ===${NC}"
run_test "ejemplos/basicos/hola_mundo.ag" "Hola Mundo" ""
run_test "ejemplos/basicos/variables_y_tipos.ag" "Variables y Tipos" "10\n5\n2.0\nÁguila\nverdadero\nfalso\n"
run_test "ejemplos/basicos/interpolacion_basica.ag" "Interpolación Básica" "Mundo\n"
run_test "ejemplos/basicos/bucle_simple.ag" "Bucle Simple" "0\n5\n"
run_test "ejemplos/basicos/bucles_debugging.ag" "Bucles Debugging" "10\n"
run_test "ejemplos/basicos/control_flujo.ag" "Control de Flujo" "10\n3\n5\n10\n"
run_test "ejemplos/basicos/prueba_vm.ag" "Prueba VM" "10\n20\n100\n50\n5\n"

echo ""
echo -e "${YELLOW}=== INTERMEDIOS ===${NC}"
run_test "ejemplos/intermedio/funciones.ag" "Funciones" "Usuario\n10\n20\n5\n5\n5\n"
run_test "ejemplos/intermedio/colecciones.ag" "Colecciones" "10\n20\n30\n40\nÁguila\n1.0\nEmerson\n2.0\n"
run_test "ejemplos/intermedio/diccionarios.ag" "Diccionarios" "Ana\n25\n26\nSantiago\n"
run_test "ejemplos/intermedio/programacion_orientada_objetos.ag" "POO" "Firulais\nMichi\n3\n"

echo ""
echo -e "${YELLOW}=== AVANZADOS ===${NC}"
run_test "ejemplos/avanzado/fibonacci.ag" "Fibonacci (JIT)" "40\n"
run_test "ejemplos/avanzado/matematicas.ag" "Matemáticas" "25\n2\n3\n1.570796\n0\n4.9\n4.1\n1\n5\n"
run_test "ejemplos/avanzado/biblioteca_estandar.ag" "Biblioteca Estándar" "  Hola Mundo  \nHola\nMundo\nÁguila\na,b,c\n,\nprueba_fs.txt\nContenido de prueba\n"
run_test "ejemplos/avanzado/modulos.ag" "Módulos" "Visitante\n10\n5\n"
run_test "ejemplos/avanzado/modulo_ejemplo.ag" "Módulo Ejemplo" ""
run_test "ejemplos/avanzado/jit_comparacion_strings.ag" "JIT Strings" "200\n"
run_test "ejemplos/avanzado/interpolacion_avanzada.ag" "Interpolación Avanzada" "10\n20\nÁguila\n"

echo ""
echo "========================================="
echo "  Resultados"
echo "========================================="
echo -e "Total:   $TOTAL"
echo -e "${GREEN}Pasados: $PASSED${NC}"
echo -e "${RED}Fallidos: $FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ Todos los tests pasaron!${NC}"
    exit 0
else
    echo -e "${RED}✗ Algunos tests fallaron${NC}"
    exit 1
fi
