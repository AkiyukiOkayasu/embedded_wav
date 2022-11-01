# embedded_wav

Rustのinclude_bytes!で埋め込んだWAVEファイルを再生する。  
将来的にはマイコンのファームウェアに効果音を埋め込む想定のもの。  
no_stdで使えるように[nom](https://github.com/Geal/nom)でWAVEのパースをしている。  

## TODO

- AIFF対応
- IMA ADPCM対応
- no_stdで動作させる
  - nomのmany0とかが使えなくなる
