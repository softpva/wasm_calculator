# **WASM CALCULATOR**
[![en](https://img.shields.io/badge/lang-en-red.svg)](./README.md)
[![pt-br](https://img.shields.io/badge/lang-pt--br-green.svg)](./README.pt-br.md)
[![es](https://img.shields.io/badge/lang-es-yellow.svg)](./README.es.md)

# Sobre
> Este é o projeto de uma calculadora com as seguintes operações básicas: +, -, *, /, **, +/-, parênteses e memória.  
> Este projeto foi construído em html-css-js e ligado a um código web assembly feito na linguagem rust.
>
> [![NPM](https://img.shields.io/npm/l/react)](./LICENSE)  

# Aspectos
> - Este é o projeto de uma calculadora com as seguintes operações básicas: +, -, *, /, **, +/-, parênteses e memória.
> - Este projeto foi construído em html-css-js vinculado a um wasm construído em linguagem rust.
> - No display superior é mostrada uma expressão infix que é formada pela inserção adequada de números, operadores, memória e parênteses.
> - A expressão acima é resolvida por um código em rust ligado a uma classe javascript por WebAssembly.(WASM)
> - WebAssembly (ou WASM) é uma linguagem binária que é convertida de forma eficiente em código de máquina pelo navegador, que a executa com muito mais eficiência do que o JavaScript.
> - A respectiva expressão infix também pode ser resolvida pela função javascript eval(), para isso basta descomentar o respectivo código e comentar a linha que contém a função solve_infix(). (a função eval() não pode resolver corretamente números negativos precedidos por operadores, a função solve_infix(), uma função em rust, pode fazer isso perfeitamente.  

# Layouts
> ## Estado Inicial  
> ![initial state](/readmeImages/init.gif)  

> ## Um exemplo  
> ![an example](/readmeImages/pict_1.gif)  

> ## Depois de colocar o resultado em memória pressioando o botão MS  
> Note que o valor é mostrado abaixo do texto MR;
> ![sore in memory](/readmeImages/pict_2.gif)  

> ## Usando o valor em memória  
> E, oviamente, você pode resgatar o valor de MR a qualquer tempo.
> ![restore from memory](/readmeImages/pict_3.gif)   

# Modelos
> - OOP
> - Functional
> - Asyncron states

# Tecnologias
> - HTML, CSS, Javascript
> - Rust
> - WASM (wasm-pack)

# Iniciando
> - Abra em qualquer navegador ou servidor.

# Versão pró
> - Calculadora Científica

# Autor
> Pedro Vitor Abreu
>
> <soft.pva@gmail.com>
>
> <https://github.com/softpva>
>