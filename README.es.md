# **WASM CALCULATOR**
[![en](https://img.shields.io/badge/lang-en-red.svg)](./README.md)
[![pt-br](https://img.shields.io/badge/lang-pt--br-green.svg)](./README.pt-br.md)
[![es](https://img.shields.io/badge/lang-es-yellow.svg)](./README.es.md)  


# Acerca de
> Este es el diseño de una calculadora con las siguientes operaciones básicas: +, -, *, /, **, +/-, paréntesis y memoria.  
> Este proyecto fue construido en html-css-js y vinculado a un código web assembly hecho en lenguaje rust.  
>
> [![NPM](https://img.shields.io/npm/l/react)](./LICENSE)  

# Aspectos
> - Este es el diseño de una calculadora con las siguientes operaciones básicas: +, -, *, /, **, +/-, paréntesis y memoria.
> - Este proyecto fue construido en html-css-js vinculado a un wasm construido en lenguaje rust.
> - La pantalla superior muestra una expresión infix que se forma insertando números, operadores, memoria y paréntesis apropiados.
> - La expresión anterior se resuelve mediante un código en rust vinculado a una clase javascript por en WebAssembly. (WASM)
> - WebAssembly (o WASM) es un lenguaje binario que el navegador convierte eficientemente en código de máquina, que lo ejecuta mucho más eficientemente que JavaScript.
> - La respectiva expresión infix también puede ser resuelta por la función javascript eval(), para eso basta con descomentar el respectivo código y comentar la línea que contiene la función solve_infix(). (La función eval() no puede resolver correctamente números negativos precedidos por operadores, la función solve_infix(), una función en rust, puede hacerlo perfectamente.  

# Diseños
> ## Estado inicial  
> ![estado inicial](/readmeImages/init.gif)

> ## Un ejemplo  
> ![un ejemplo](/readmeImages/pict_1.gif)

> ## Después de colocar el resultado en la memoria presionando el botón MS  
> Tenga en cuenta que el valor se muestra debajo del texto MR.  
> ![me duele la memoria](/readmeImages/pict_2.gif)

> ## Usando el valor en memoria  
> Y por supuesto, puedes canjear el valor de MR en cualquier  momento.
> ![restaurar desde memoria](/readmeImages/pict_3.gif)

# Modelos
> - OOP
> - Functional
> - Asyncron states

# Tecnologias
> - HTML, CSS, Javascript
> - Rust
> - WASM (wasm-pack)

# Iniciando
> - Abrir en cualquier navegador o servidor.  

# Versão pró
> - Calculadora Científica

# Autor
> Pedro Vitor Abreu
>
> <soft.pva@gmail.com>
>
> <https://github.com/softpva>
>