# BufRead_sample

[[Rust] 引数の有無で read 対象をファイル・標準入力に変更する]()の説明で使用したプログラムです。  

## 概要

ファイル・標準入力の内容を、行数とともに表示します。  

* プログラムの引数にファイルが指定されている場合、ファイルの内容を読み込む。
* プログラムの引数にファイルが指定されていない場合、標準入力の内容を読み込む。

という処理をします。  

## 実行例

* プログラムの引数にファイル名が指定されている場合、ファイルの内容を表示します。  

    ```shell
    $ cat test.txt
    aaa
    bbb
    ccc
    ddd
    eee
    fff
    $ cargo run test.txt
        Finished dev [unoptimized + debuginfo] target(s) in 0.02s
        Running `target/debug/bufread_sample test.txt`
    1: aaa
    2: bbb
    3: ccc
    4: ddd
    5: eee
    6: fff
    ```

* プログラムの引数に何も指定されていない場合は、標準入力の内容を表示します。  

    ```shell
    $ echo -e "aaa\nbbb\nccc\nddd\neee\nfff" | cargo run
        Finished dev [unoptimized + debuginfo] target(s) in 0.02s
        Running `target/debug/bufread_sample`
    1: aaa
    2: bbb
    3: ccc
    4: ddd
    5: eee
    6: fff
    ```