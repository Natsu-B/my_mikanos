# mikanosをrustを用いて実装しようとしています
と言ってももうだいぶ逸れちゃってる気がしますが、、、
<!---## 一部のプログラムはMilvusVisorからのコピペとなっています。--->

今のところ、aarch64のuefi対応となっています。

## 目標
- arch依存の部分をできるだけ少なくする
    - 基本的にアセンブリ等のarch依存コードはarch以下のフォルダーにまとめる
        - これは [Methylenix](https://github.com/PG-MANA/Methylenix) を参考にさせてもらっています
- オプションでdebug用か否かを指定できるようにして、uartからdebug情報を出力できるようにする。