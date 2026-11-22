# Espiral Arquimediana Composta por Primos

Um visualizador de dados escrito em Rust com [raylib](https://github.com/raylib-rs/raylib-rs), inspirado em um [vídeo do 3Blue1Brown](https://youtu.be/EK32jo7i5LQ?si=ZehKKW_4DroxbZxK) e em um sonho que tive.

<div align="center">
  <img src="./assets/demo.gif" alt="Espiral girando" width="500">
  
  <samp>(simulação acelerada)</samp>
</div>

O programa exibe padrões entre números primos numa espécie de espiral arquimediana. Cada primo `p` é desenhado nas coordenadas polares `(r, θ) = (p, p)`.

```
x = p·cos(p)
y = -p·sin(p)
```

Os primos são gerados pelo [crivo de Eratóstenes](https://pt.wikipedia.org/wiki/Crivo_de_Erat%C3%B3stenes).

> Para Ellen <3

## Controles

| Tecla                    | Ação                                |
| ------------------------ | ----------------------------------- |
| `Espaço`                 | Pausar/retomar                      |
| `A`                      | Recomeçar do início                 |
| `S`                      | Mostrar todos os primos             |
| `Z`                      | Alternar zoom automático            |
| `X`                      | Alternar rotação                    |
| `R`                      | Alternar inversão                   |
| `P`                      | Alternar otimização com pixels      |
| `T`                      | Trocar tema                         |
| `H`                      | Alternar HUD                        |
| `B`                      | Alternar dicas                      |
| `F11`                    | Alternar tela cheia                 |
| `Roda do mouse`          | Aproximar/afastar                   |
| `Cima`, `Baixo`          | Aproximar/afastar                   |
| `Ctrl` + zoom            | Zoom mais preciso                   |
| `Alt esquerdo` (segurar) | Mostrar o cursor                    |
| `Q`                      | Sair                                |

## Compilar e executar

Com Nix:

```sh
nix-shell
cargo run --release
```

Sem Nix, instale uma toolchain do Rust e as dependências de compilação do raylib (cmake, libclang, bibliotecas de desenvolvimento X11/GL, etc.), e então execute `cargo run --release`.

## Configuração

Constantes no início de `src/main.rs`:

| Constante                              | Significado                                  |
| -------------------------------------- | -------------------------------------------- |
| `SIEVE_LIMIT`                          | Maior número a ser verificado no crivo       |
| `INITIAL_VISIBLE_PRIMES`               | Primos visíveis ao iniciar                   |
| `PRIMES_INCREMENT`                     | Primos adicionados por quadro                |
| `INITIAL_ZOOM_LEVEL`                   | Zoom ao iniciar e após recomeçar             |
| `MIN_ZOOM`, `MAX_ZOOM`                 | Limites de zoom                              |
| `ROTATION_SPEED`                       | Velocidade de rotação da espiral             |
| `BASE_SQUARE_SIZE`                     | Tamanho de cada ponto                        |
| `WINDOW_WIDTH`, `WINDOW_HEIGHT`        | Tamanho inicial da janela                    |

Os temas são definidos em `Theme::colors()`.
