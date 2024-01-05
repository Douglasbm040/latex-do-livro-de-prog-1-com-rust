# Livro de Programação em Rust para Iniciantes

Este projeto consiste em um livro voltado para o ensino de programação na disciplina de Programação 1 na Universidade Federal do Espírito Santo (UFES). O objetivo é proporcionar uma introdução abrangente à linguagem de programação Rust, focando em conceitos fundamentais de programação e boas práticas de desenvolvimento.

## Pré-requisitos

Antes de executar os exemplos de código e visualizar o conteúdo deste livro, certifique-se de ter as seguintes ferramentas instaladas em seu sistema:

1. **Visual Studio Code:** Recomendamos a utilização do Visual Studio Code como ambiente de desenvolvimento. Você pode baixá-lo em [https://code.visualstudio.com/](https://code.visualstudio.com/).

2. **Extensão LaTeX Workshop:** Instale a extensão LaTeX Workshop no Visual Studio Code para compilar e visualizar documentos LaTeX. Você pode encontrar a extensão em [https://marketplace.visualstudio.com/items?itemName=James-Yu.latex-workshop](https://marketplace.visualstudio.com/items?itemName=James-Yu.latex-workshop).

3. **MiKTeX:** O MiKTeX é um sistema de distribuição TeX/LaTeX para Windows. Baixe e instale o MiKTeX a partir do site oficial em [https://miktex.org/download](https://miktex.org/download).
4. **Perl:**
   - **Windows:**
     - Strawberry Perl: Baixe o instalador no [site oficial](http://strawberryperl.com/) e siga as instruções na tela.
     - ActivePerl: Baixe o instalador no [site oficial](https://www.activestate.com/products/perl/downloads/) e siga as instruções na tela.
   - **macOS:**
     - Homebrew: Abra o Terminal e execute:
       ```bash
       /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
       brew install perl
       ```
     - MacPorts: Abra o Terminal e execute:
       ```bash
       sudo port install perl5
       ```
   - **Linux (Ubuntu/Debian):**
     ```bash
     sudo apt-get update
     sudo apt-get install perl
     ```
   - **Outras Distribuições Linux:**
     Utilize o gerenciador de pacotes específico da sua distribuição para instalar o Perl.

## Executando o Projeto

Siga os passos abaixo para executar o projeto em seu ambiente local:

1. **Clone o repositório:**
   ```bash
   git clone https://github.com/seu-usuario/latex-do-livro-de-prog-1-com-rust.git
   ```

2. **Abra o projeto no Visual Studio Code:**
   ```bash
   cd latex-do-livro-de-prog-1-com-rust
   code .
   ```

3. **Compile o Livro:**
   - Abra o arquivo principal do livro `dissertacao.tex` no Visual Studio Code.
   - Utilize a extensão LaTeX Workshop para compilar o documento.

4. **Visualize o Livro:**
   - Após a compilação, abra o documento resultante  para visualizar o livro.
  
## Estrutura do projeto 

O projeto está estruturado em :

1. **código**: Para exemplo de codigos usados no livro
2. **dependece**: Para bibliotecas externas usadas no livro
3. **images**: Para imagens usadas no livro
4. **dissertacao**: arquivo principal do livro   

## Contribuindo

Se desejar contribuir para o desenvolvimento deste livro, sinta-se à vontade para abrir issues, enviar pull requests ou entrar em contato com a equipe de desenvolvimento.

Esperamos que este livro seja uma ferramenta valiosa para o aprendizado de Rust na disciplina de Programação 1 na UFES. Boa leitura e bons estudos!


